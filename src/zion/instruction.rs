pub struct Instruction {
    id: Id, //mov
    type_: Type, //R_R(mov A,B)
    arg1: u8,
    arg2: u8,
}

impl Instruction {
    //pub fn new(id: u8, atype: u8, arg1: u8, arg2: u8) -> Instruction {
    pub fn new(instr_bytes: &[u8]) -> Instruction {
        Instruction {
            //program_line.as_bytes()[0],
            id: Id::from_u8(instr_bytes[0]).unwrap(),
            type_: Type::from_u8(instr_bytes[1]).unwrap(),
            arg1: instr_bytes[2],
            arg2: instr_bytes[3],
        }
    }
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn type_(&self) -> Type {
        self.type_
    }

    pub fn arg1(&self) -> u8 {
        self.arg1
    }

    pub fn arg2(&self) -> u8 {
        self.arg2
    }
}

//instruction id
#[derive(Copy, Clone)]
pub enum Id {
    //mov A, B(RR)
    //mov A, 0xFF(R, IMM)
    //mov A, @0xFF(R, MEM)
    Stp = 0,
    Add,
    Sub,
    Ror,
    Rol,
    Inc,
    Dec,
    Jmp,
    Jz,
    Jnz,
    Jc,
    Jnc,
    Cmp,
    Tm1,
    Tm2,
    And,
    Or,
    Not,
    Xor,
    Mov,
    Nop,
    Sec,
    Clc,
    In,
    Out,
    Swap,
}

impl Id {
    pub fn from_u8(n: u8) -> Option<Id> {
        match n {
            0 => Some(Id::Stp),
            1 => Some(Id::Add),
            2 => Some(Id::Sub),
            3 => Some(Id::Ror),
            4 => Some(Id::Rol),
            5 => Some(Id::Inc),
            6 => Some(Id::Dec),
            7 => Some(Id::Jmp),
            8 => Some(Id::Jz),
            9 => Some(Id::Jnz),
            0xa => Some(Id::Jc),
            0xb => Some(Id::Jnc),
            0xc => Some(Id::Cmp),
            0xd => Some(Id::Tm1),
            0xe => Some(Id::Tm2),
            0xf => Some(Id::And),
            0x10 => Some(Id::Or),
            0x11 => Some(Id::Not),
            0x12 => Some(Id::Xor),
            0x13 => Some(Id::Mov),
            0x14 => Some(Id::Nop),
            0x15 => Some(Id::Sec),
            0x16 => Some(Id::Clc),
            0x17 => Some(Id::In),
            0x18 => Some(Id::Out),
            0x19 => Some(Id::Swap),
            _ => None,
        }
    }
}

//instruction type
#[derive(Copy, Clone)]
pub enum Type {
    R = 0, //inc A
    IMM, //jmp 0xFF
    R_R, //mov A, B
    R_IMM, //mov A, 0xFF
    R_MEM, //mov A, @0xFF
}

impl Type {
    pub fn from_u8(n: u8) -> Option<Type> {
        match n {
            0 => Some(Type::R),
            1 => Some(Type::IMM),
            2 => Some(Type::R_R),
            3 => Some(Type::R_IMM),
            4 => Some(Type::R_MEM),
            _ => None,
        }
    }
}
