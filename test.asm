
// replace to 256
@256
D=A
@SP
M=D

@892
D=A
@SP
A=M
M=D
@SP
M=M+1

@891
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_lt_1
D;JLT
@RETURNFALSE_lt_1
D;JGE

(RETURNTRUE_lt_1)
D=-1
@RETURNEND_lt_1
0;JMP
(RETURNFALSE_lt_1)
D=0
(RETURNEND_lt_1)
@SP
A=M
M=D
@SP
M=M+1

@891
D=A
@SP
A=M
M=D
@SP
M=M+1

@892
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_lt_2
D;JLT
@RETURNFALSE_lt_2
D;JGE

(RETURNTRUE_lt_2)
D=-1
@RETURNEND_lt_2
0;JMP
(RETURNFALSE_lt_2)
D=0
(RETURNEND_lt_2)
@SP
A=M
M=D
@SP
M=M+1

@891
D=A
@SP
A=M
M=D
@SP
M=M+1

@891
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_lt_3
D;JLT
@RETURNFALSE_lt_3
D;JGE

(RETURNTRUE_lt_3)
D=-1
@RETURNEND_lt_3
0;JMP
(RETURNFALSE_lt_3)
D=0
(RETURNEND_lt_3)
@SP
A=M
M=D
@SP
M=M+1

@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_gt_4
D;JGT
@RETURNFALSE_gt_4
D;JLE

(RETURNTRUE_gt_4)
D=-1
@RETURNEND_gt_4
0;JMP
(RETURNFALSE_gt_4)
D=0
(RETURNEND_gt_4)
@SP
A=M
M=D
@SP
M=M+1

@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_gt_5
D;JGT
@RETURNFALSE_gt_5
D;JLE

(RETURNTRUE_gt_5)
D=-1
@RETURNEND_gt_5
0;JMP
(RETURNFALSE_gt_5)
D=0
(RETURNEND_gt_5)
@SP
A=M
M=D
@SP
M=M+1

@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

@SP
AM=M-1
D=M
@SP
AM=M-1

D=M-D
@RETURNTRUE_gt_6
D;JGT
@RETURNFALSE_gt_6
D;JLE

(RETURNTRUE_gt_6)
D=-1
@RETURNEND_gt_6
0;JMP
(RETURNFALSE_gt_6)
D=0
(RETURNEND_gt_6)
@SP
A=M
M=D
@SP
M=M+1