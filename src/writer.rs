use std::{
    fmt::format,
    fs::File,
    io::{BufWriter, Write},
    str::FromStr,
};

use crate::parser::CommandType;


static ADD_ASM: &str = r"
// pop to save register (AM=M-1, M=D+M)
@SP
AM=M-1
D=M
// pop
@SP
AM=M-1
// add
M=M+D
@SP
M=M+1
";

static SUB_ASM: &str = r"
// pop to save register (AM=M-1, M=D+M)
@SP
AM=M-1
D=M
// pop
@SP
AM=M-1
// add
M=M-D
@SP
M=M+1
";

static NEG_ASM: &str = r"
@SP
AM=M-1
// neg
M=-M
@SP
M=M+1
";

static AND_ASM: &str = r"
// and
// pop to save register (AM=M-1, M=D+M)
@SP
AM=M-1
D=M
// pop
@SP
AM=M-1
// and
M=M&D
@SP
M=M+1
";

static OR_ASM: &str = r"
// and
// pop to save register (AM=M-1, M=D+M)
@SP
AM=M-1
D=M
// pop
@SP
AM=M-1
// or
M=M|D
@SP
M=M+1
";

static NOT_ASM: &str = r"
@SP
AM=M-1
// not 
M=!M
@SP
M=M+1
";

static PUSH_CONST_AMS: &str = "
@{}
D=A
@SP
A=M
M=D
@SP
M=M+1
";

static PUSH_STATIC_AMS: &str = "
@{}
D=M
@SP
A=M
M=D
@SP
M=M+1
";

static POP_STATIC_AMS: &str = "
@SP
AM=M-1
D=M
@{}
M=D
";

static EQ_CONST_ASM: &str = "
@SP
AM=M-1
D=M
@SP
AM=M-1

D=D-M
@RETURNTRUE
D;JEQ
@RETURNFALSE
D;JNE

(RETURNTRUE)
D=-1
@RETURNEND
0;JMP
(RETURNFALSE)
D=0
(RETURNEND)
@SP
A=M
M=D
@SP
M=M+1
";

static LT_CONST_ASM: &str = "
@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE
D;JLT
@RETURNFALSE
D;JGE

(RETURNTRUE)
D=-1
@RETURNEND
0;JMP
(RETURNFALSE)
D=0
(RETURNEND)
@SP
A=M
M=D
@SP
M=M+1
";

static GT_CONST_ASM: &str = "
@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE
D;JGT
@RETURNFALSE
D;JLE

(RETURNTRUE)
D=-1
@RETURNEND
0;JMP
(RETURNFALSE)
D=0
(RETURNEND)
@SP
A=M
M=D
@SP
M=M+1
";

static RET_TRUE_LABEL: &str = "RETURNTRUE";
static RET_FALSE_LABEL: &str = "RETURNFALSE";
static RET_END_LABEL: &str = "RETURNEND";
pub struct CodeWriter<W: std::io::Write> {
    f: BufWriter<W>,
    logical_op_count: usize,
    filename: String,
}

fn generate_pop_specified_register_template(index: usize, reg_name: &str) -> String {
    format!(
        r###"
@{index}
D=A
@{reg_name}
D=M+D
@R13
M=D

@SP
AM=M-1
D=M
@R13
A=M
M=D

"###,
    )
}

fn generate_push_specified_register_template(index: usize, reg_name: &str) -> String {
    format!(
        r###"
@{index}
D=A
@{reg_name}
D=M+D
@R13
M=D

@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1

"###,
    )
}

fn generate_pop_specified_register_template_pointer(index: usize, reg_name: &str) -> String {
    format!(
        r###"
@{index}
D=A
@{reg_name}
D=A+D
@R13
M=D

@SP
AM=M-1
D=M
@R13
A=M
M=D
"###,
    )
}
fn generate_push_specified_register_template_pointer(index: usize, reg_name: &str) -> String {
    format!(
        r###"
@{index}
D=A
@{reg_name}
D=A+D
@R13
M=D

@R13
A=M
D=M
@SP
A=M
M=D
@SP
M=M+1
"###,
    )
}

static INIT: &str = "
// replace to 256
@256
D=A
@SP
M=D

// set to ARG reg
@400
D=A
@ARG
M=D
";

#[derive(strum_macros::EnumString)]
enum Segment {
    #[strum(serialize = "constant")]
    constant,
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "argument")]
    Arg,
    #[strum(serialize = "that")]
    That,
    #[strum(serialize = "this")]
    This,
    #[strum(serialize = "temp")]
    Temp,
    #[strum(serialize = "pointer")]
    Pointer,
}

impl Segment {
    fn write_push_asm<W>(&self, index: i64, f: &mut BufWriter<W>, label: Option<&str>)
    where
        W: std::io::Write,
    {
        match self {
            Segment::constant => {
                let replaced_str = PUSH_CONST_AMS.replace("{}", &index.to_string());
                f.write_all(replaced_str.as_bytes()).unwrap();
            }
            Segment::Static => {
                let replaced_str = PUSH_STATIC_AMS
                    .replace("{}", &format!("{}.{}", label.unwrap(), &index.to_string()));
                f.write_all(replaced_str.as_bytes()).unwrap();
            }
            Segment::Local | &Segment::Arg | Segment::That | Segment::This => {
                f.write_all(
                    generate_push_specified_register_template(
                        index as usize,
                        self.get_register_name(),
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
            Segment::Temp | Segment::Pointer => {
                f.write_all(
                    generate_push_specified_register_template_pointer(
                        index as usize,
                        self.get_register_name(),
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
    }

    fn write_pop_asm<W>(&self, index: i64, f: &mut BufWriter<W>, label: Option<&str>)
    where
        W: std::io::Write,
    {
        match self {
            Segment::constant => {
                panic!("cannot pop constant")
            }
            Segment::Static => {
                let replaced_str = POP_STATIC_AMS
                    .replace("{}", &format!("{}.{}", label.unwrap(), &index.to_string()));
                f.write_all(replaced_str.as_bytes()).unwrap();
            }
            Segment::Local | &Segment::Arg | Segment::That | Segment::This => {
                f.write_all(
                    generate_pop_specified_register_template(
                        index as usize,
                        self.get_register_name(),
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
            Segment::Temp | Segment::Pointer => {
                f.write_all(
                    generate_pop_specified_register_template_pointer(
                        index as usize,
                        self.get_register_name(),
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
    }

    fn get_register_name(&self) -> &str {
        match self {
            Segment::constant => todo!(),
            Segment::Static => todo!(),
            Segment::Local => "LCL",
            Segment::Arg => "ARG",
            Segment::That => "THAT",
            Segment::This => "THIS",
            Segment::Temp => "5",
            Segment::Pointer => "3",
        }
    }
}


impl<W: std::io::Write> CodeWriter<W> {
    pub fn new(output: BufWriter<W>) -> CodeWriter<W> {
        CodeWriter {
            f: output,
            logical_op_count: 0,
            filename: String::new(),
        }
    }

    pub fn setFileName(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn debug(&mut self) {
        self.f.write_all(INIT.as_bytes()).unwrap();
    }

    fn replace_logical_op(
        &self,
        true_label: &str,
        false_label: &str,
        end_label: &str,
        command: &str,
        template: &str,
    ) -> String {
        let mut replaced_true = true_label.to_string();
        replaced_true.push_str(&format!("_{}_{}", command, self.logical_op_count));
        let mut replaced_false = false_label.to_string();
        replaced_false.push_str(&format!("_{}_{}", command, self.logical_op_count));
        let mut replaced_end = end_label.to_string();
        replaced_end.push_str(&format!("_{}_{}", command, self.logical_op_count));

        template
            .replace(true_label, &replaced_true)
            .replace(false_label, &replaced_false)
            .replace(end_label, &replaced_end)
    }

    pub fn writeArithmetic(&mut self, command: &str) {
        if command == "add" {
            self.f.write_all(ADD_ASM.as_bytes()).unwrap();
        } else if command == "sub" {
            self.f.write_all(SUB_ASM.as_bytes()).unwrap();
        } else if command == "neg" {
            self.f.write_all(NEG_ASM.as_bytes()).unwrap();
        } else if command == "and" {
            self.f.write_all(AND_ASM.as_bytes()).unwrap();
        } else if command == "or" {
            self.f.write_all(OR_ASM.as_bytes()).unwrap();
        } else if command == "not" {
            self.f.write_all(NOT_ASM.as_bytes()).unwrap();
        } else if command == "eq" {
            self.logical_op_count += 1;
            let replaced_eq_asm = self.replace_logical_op(
                RET_TRUE_LABEL,
                RET_FALSE_LABEL,
                RET_END_LABEL,
                command,
                EQ_CONST_ASM,
            );
            self.f.write_all(replaced_eq_asm.as_bytes()).unwrap();
        } else if command == "lt" {
            self.logical_op_count += 1;
            let replaced_eq_asm = self.replace_logical_op(
                RET_TRUE_LABEL,
                RET_FALSE_LABEL,
                RET_END_LABEL,
                command,
                LT_CONST_ASM,
            );
            self.f.write_all(replaced_eq_asm.as_bytes()).unwrap();
        } else if command == "gt" {
            self.logical_op_count += 1;
            let replaced_eq_asm = self.replace_logical_op(
                RET_TRUE_LABEL,
                RET_FALSE_LABEL,
                RET_END_LABEL,
                command,
                GT_CONST_ASM,
            );
            self.f.write_all(replaced_eq_asm.as_bytes()).unwrap();
        }
    }

    pub fn writePushPop(&mut self, command: &CommandType, segment: &str, index: i64) {
        let seg = Segment::from_str(segment).unwrap();
        match command {
            CommandType::C_PUSH => {
                seg.write_push_asm(index, &mut self.f, Some(&self.filename));
            }
            CommandType::C_POP => {
                seg.write_pop_asm(index, &mut self.f, Some(&self.filename));
            }
            _ => {
                panic!("not called this command type {:?}", command)
            }
        }
    }
}

impl<W: std::io::Write> Drop for CodeWriter<W> {
    fn drop(&mut self) {
        self.f.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufWriter};

    use super::CodeWriter;

    #[test]
    fn work_test() {
        let file = File::create("dump.asm").unwrap();
        let mut writer = CodeWriter::new(BufWriter::new(file));
        writer.writePushPop(&crate::parser::CommandType::C_PUSH, "constant", 7);
        writer.writePushPop(&crate::parser::CommandType::C_PUSH, "constant", 8);
        writer.writeArithmetic("add");
    }
}
