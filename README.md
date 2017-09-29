# zion-vm
zion-vm is a virtual machine made using rust-lang for the zion computer architecture

-----------------

<div align="center">
  <img src="https://github.com/jrabello/zion-vm/raw/master/img/state.png"><br><br>
</div>

-----------------

## zion instruction set
### STP (opcode: 0x00 0x00 0x00 0x00) (size 4)
stops program execution, prints machine state and terminates

### MOV (opcode 0x13 0x00 0x00 0x00) (size 4)
move data, register or memory address to register

types:
      ```
       mov R, R ;(type 0x02)

       mov R, IMM ;(type 0x03)

       mov R, @MEM ;(type 0x04)
       ```
