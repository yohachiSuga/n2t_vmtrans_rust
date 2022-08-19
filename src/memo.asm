// push constant 7
// push constant 8
// add

// replace to 256
@256
D=A
@SP
M=D

// push constant
@7
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant
@8
D=A
@SP
A=M
M=D
@SP
M=M+1

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



// // add
// // pop to save register (AM=M-1, M=D+M)
// @SP
// AM=M-1
// D=M
// // pop
// @SP
// AM=M-1
// // add
// M=M+D
// @SP
// M=M+1

// // eq
// @SP
// AM=M-1
// D=M
// @SP
// AM=M-1

// D=D-M
// @RETURNTRUE
// D;JEQ
// @RETURNFALSE
// D;JNE

// (RETURNTRUE)
// @0
// D=A
// @RETURNEND
// 0;JMP
// (RETURNFALSE)
// @-1
// D=A
// (RETURNEND)
// @SP
// A=M
// M=D
// @SP
// M=M+1

// // lt
// @SP
// AM=M-1
// D=M
// @SP
// AM=M-1

// D=D-M
// @RETURNTRUE
// D;JLT
// @RETURNFALSE
// D;JGE

// (RETURNTRUE)
// @0
// D=A
// @RETURNEND
// 0;JMP
// (RETURNFALSE)
// @1111111111111111
// D=A
// (RETURNEND)
// @SP
// A=M
// M=D
// @SP
// M=M+1

// // gt
// @SP
// AM=M-1
// D=M
// @SP
// AM=M-1

// D=D-M
// @RETURNTRUE
// D;JGT
// @RETURNFALSE
// D;JLE

// (RETURNTRUE)
// @0
// D=A
// @RETURNEND
// 0;JMP
// (RETURNFALSE)
// @1111111111111111
// D=A
// (RETURNEND)
// @SP
// A=M
// M=D
// @SP
// M=M+1

// push static 3
@Xxx.3
D=M
@SP
A=M
M=D
@SP
M=M+1

// pop static 3
@SP
AM=M-1
D=M
@Xxx.3
M=D