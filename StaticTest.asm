
@111
D=A
@SP
A=M
M=D
@SP
M=M+1

@333
D=A
@SP
A=M
M=D
@SP
M=M+1

@888
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@StaticTest.vm.8
M=D

@SP
AM=M-1
D=M
@StaticTest.vm.3
M=D

@SP
AM=M-1
D=M
@StaticTest.vm.1
M=D

@StaticTest.vm.3
D=M
@SP
A=M
M=D
@SP
M=M+1

@StaticTest.vm.1
D=M
@SP
A=M
M=D
@SP
M=M+1

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

@StaticTest.vm.8
D=M
@SP
A=M
M=D
@SP
M=M+1

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
