extern crate num_traits;
use num_traits::FromPrimitive;


pub struct Instruction {
    id: Id, //mov
    type_: Type, //R_R(mov A,B)
    arg1: u8,
    arg2: u8,
    size: u8,
}

impl Instruction {
    pub fn new(instr_byte: u8) -> Instruction {
        Instruction {
            id: Id::from_u8(instr_byte).unwrap(),
            type_: Type::from_u8(0).unwrap(),
            arg1: 0,
            arg2: 0,
            size: 0,
        }
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_type(&self) -> Type {
        self.type_
    }
    pub fn set_type(&mut self, type_: u8) {
        self.type_ = Type::from_u8(type_).unwrap();
    }

    pub fn get_arg1(&self) -> u8 {
        self.arg1
    }

    pub fn set_arg1(&mut self, arg1: u8) {
        self.arg1 = arg1;
    }

    pub fn set_arg2(&mut self, arg2: u8) {
        self.arg2 = arg2;
    }

    pub fn get_arg2(&self) -> u8 {
        self.arg2
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }

    pub fn set_size(&mut self, size: u8) {
        self.size = size
    }
}

//instruction id
#[repr(u8)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum Id {
    //mov A, B(RR)
    //mov A, 0xFF(R, IMM)
    //mov A, @0xFF(R, MEM)
    Stp = 0,
    Add = 1,
    Sub = 2,
    Ror = 3,
    Rol = 4,
    Inc = 5,
    Dec = 6,
    Jmp = 7,
    Jz = 8,
    Jnz = 9,
    Jc = 0xa,
    Jnc = 0xb,
    Cmp = 0xc,
    Tm1 = 0xd,
    Tm2 = 0xe,
    And = 0xf,
    Or = 0x10,
    Not = 0x11,
    Xor = 0x12,
    Mov = 0x13,
    Nop = 0x14,
    Sec = 0x15,
    Clc = 0x16,
    In = 0x17,
    Out = 0x18,
    Swap = 0x19,
}

///instruction type
#[repr(u8)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum Type {
    R = 0, //inc A
    IMM = 1, //jmp 0xFF
    R_R = 2, //mov A, B
    R_IMM = 3, //mov A, 0xFF
    R_MEM = 4, //mov A, @0xFF
}
