use std::{
    io::{BufWriter, Write},
    str::FromStr,
};

use crate::{
    parser::CommandType,
    template::{
        generate_pop_specified_register_template, generate_pop_specified_register_template_pointer,
        generate_push_specified_register_template,
        generate_push_specified_register_template_pointer, ADD_ASM, AND_ASM, CMP_CONST_ASM,
        DEF_LABEL_AMS, FALSE_CMP_LABEL, GOTO_LABEL_AMS, IFGOTO_LABEL_AMS, INIT, LABEL_NAME,
        NEG_ASM, NOT_ASM, OR_ASM, POP_STATIC_AMS, PUSH_CONST_AMS, PUSH_STATIC_AMS, RET_END_LABEL,
        RET_FALSE_LABEL, RET_TRUE_LABEL, SUB_ASM, TRUE_CMP_LABEL,
    },
};

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

pub struct CodeWriter<W: std::io::Write> {
    f: BufWriter<W>,
    logical_op_count: usize,
    filename: String,
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

    fn generate_cmp_template(&mut self, command: &str) -> String {
        self.logical_op_count += 1;
        let template = CMP_CONST_ASM;
        let true_label = RET_TRUE_LABEL;
        let false_label = RET_FALSE_LABEL;
        let end_label = RET_END_LABEL;
        let true_cmp_label = TRUE_CMP_LABEL;
        let false_cmp_label = FALSE_CMP_LABEL;

        let mut replaced_true = true_label.to_string();
        replaced_true.push_str(&format!("_{}_{}", command, self.logical_op_count));
        let mut replaced_false = false_label.to_string();
        replaced_false.push_str(&format!("_{}_{}", command, self.logical_op_count));
        let mut replaced_end = end_label.to_string();
        replaced_end.push_str(&format!("_{}_{}", command, self.logical_op_count));

        let (true_cmp, false_cmp) = {
            if command == "eq" {
                ("JEQ", "JNE")
            } else if command == "lt" {
                ("JLT", "JGE")
            } else if command == "gt" {
                ("JGT", "JLE")
            } else {
                panic!("not supported command {}", command)
            }
        };

        template
            .replace(true_label, &replaced_true)
            .replace(false_label, &replaced_false)
            .replace(end_label, &replaced_end)
            .replace(true_cmp_label, true_cmp)
            .replace(false_cmp_label, false_cmp)
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
        } else {
            let cmp_asm = self.generate_cmp_template(command);
            self.f.write_all(cmp_asm.as_bytes()).unwrap();
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

    pub fn writeLabel(&mut self, command: &CommandType, label: &str) {
        let asm = DEF_LABEL_AMS.replace(LABEL_NAME, label);
        self.f.write_all(asm.as_bytes()).unwrap();
    }
    pub fn writeGoto(&mut self, command: &CommandType, label: &str) {
        let asm = GOTO_LABEL_AMS.replace(LABEL_NAME, label);
        self.f.write_all(asm.as_bytes()).unwrap();
    }
    pub fn writeIf(&mut self, command: &CommandType, label: &str) {
        let asm = IFGOTO_LABEL_AMS.replace(LABEL_NAME, label);
        self.f.write_all(asm.as_bytes()).unwrap();
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
