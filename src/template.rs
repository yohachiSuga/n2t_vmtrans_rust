pub static ADD_ASM: &str = r"
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

pub static SUB_ASM: &str = r"
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

pub static NEG_ASM: &str = r"
@SP
AM=M-1
// neg
M=-M
@SP
M=M+1
";

pub static AND_ASM: &str = r"
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

pub static OR_ASM: &str = r"
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

pub static NOT_ASM: &str = r"
@SP
AM=M-1
// not 
M=!M
@SP
M=M+1
";

pub static PUSH_CONST_AMS: &str = "
@{}
D=A
@SP
A=M
M=D
@SP
M=M+1
";

pub static PUSH_STATIC_AMS: &str = "
@{}
D=M
@SP
A=M
M=D
@SP
M=M+1
";

pub static POP_STATIC_AMS: &str = "
@SP
AM=M-1
D=M
@{}
M=D
";

pub static EQ_CONST_ASM: &str = "
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

pub static LT_CONST_ASM: &str = "
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

pub static GT_CONST_ASM: &str = "
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

pub static RET_TRUE_LABEL: &str = "RETURNTRUE";
pub static RET_FALSE_LABEL: &str = "RETURNFALSE";
pub static RET_END_LABEL: &str = "RETURNEND";

pub fn generate_pop_specified_register_template(index: usize, reg_name: &str) -> String {
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

pub fn generate_push_specified_register_template(index: usize, reg_name: &str) -> String {
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

pub fn generate_pop_specified_register_template_pointer(index: usize, reg_name: &str) -> String {
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
pub fn generate_push_specified_register_template_pointer(index: usize, reg_name: &str) -> String {
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

pub static INIT: &str = "
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
