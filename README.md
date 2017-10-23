# zion-vm
zion-vm is a virtual machine made using rust-lang for the zion computer architecture

<div align="center">
  <img src="https://github.com/jrabello/zion-vm/raw/master/img/state.png"><br><br>
</div>

-----------------

## zion instruction set
#### STP (0x00) (size 1)
stops program execution, prints machine state and terminates

#### MOV (0x13 0x00 0x00 0x00) (opcode type op1 op2) (size 4)
move data, register or memory address to register

types:
```
mov R, R
mov R, IMM
mov R, @MEM
mov @MEM, R
```
