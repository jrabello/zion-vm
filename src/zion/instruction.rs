extern crate num_traits;
use num_traits::FromPrimitive;


pub struct Instruction {
    id: Id, //mov
    type_: Type, //R_R(mov A,B)
    op1: u8,
    op2: u8,
    size: u8,
}

impl Instruction {
    pub fn new(instr_byte: u8) -> Instruction {
        let instr_id = match Id::from_u8(instr_byte) {
            Some(id) => id,
            None => panic!("unknown instruction!"),
        };
        Instruction {
            id: instr_id,
            type_: Type::from_u8(0).unwrap(),
            op1: 0,
            op2: 0,
            size: 1,
        }
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_type(&self) -> Type {
        self.type_
    }
    pub fn set_type(&mut self, type_: u8) {
        self.size += 1;
        self.type_ = Type::from_u8(type_).unwrap();
    }

    pub fn get_op1(&self) -> u8 {
        self.op1
    }
    pub fn set_op1(&mut self, op1: u8) {
        self.size += 1;
        self.op1 = op1;
    }

    pub fn set_op2(&mut self, op2: u8) {
        self.size += 1;
        self.op2 = op2;
    }
    pub fn get_op2(&self) -> u8 {
        self.op2
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
    MEM = 2, //jmp @0xFF
    R_R = 3, //mov A, B
    R_IMM = 4, //mov A, 0xFF
    R_MEM = 5, //mov A, @0xFF
    MEM_R = 6, //mov @0xFF, A
}
