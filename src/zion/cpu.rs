use std;
use zion;
use zion::Instruction;
use zion::Registers;
use zion::Flags;
use zion::Cache;
use zion::Io;

pub struct Cpu {
    pub io: Io,
    pub registers: Registers,
    pub flags: Flags,
    pub cache: Cache,
    pub i_size: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            flags: Flags::new(),
            cache: Cache::new(),
            io: Io::new(),
            i_size: 4,
        }
    }

    pub fn init(&mut self, program_lines: &str) {
        //copying program to cache(code)
        self.cache.code_copy(program_lines);

        //cpu pipeline
        loop {
            let current_instruction;
            {
                let bytes = self.fetch();
                current_instruction = self.decode(bytes);
            }
            self.execute(current_instruction);
        }
    }

    fn fetch(&self) -> &[u8] {
        //get current instruction from cache(code)
        let idx = self.registers.ip() as usize;
        self.cache.code_at(idx)
    }

    fn decode(&self, bytes: &[u8]) -> Instruction {
        //decode instruction
        Instruction::new(bytes)
    }

    fn execute(&mut self, instr: Instruction) {
        //execute instruction
        match instr.id() {         
            zion::instruction::Id::Mov => {
                //println!("{}", "mov!");
                match instr.type_() {
                    zion::instruction::Type::R_R => {
                        //move register to register
                        //println!("type:R_R {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.arg1();
                        let reg2 = instr.arg2();
                        self.registers[reg1] = self.registers[reg2];
                    }
                    zion::instruction::Type::R_IMM => {
                        //move to register IMM value
                        //println!("type:R_IMM {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.arg1();
                        let data = instr.arg2();
                        self.registers[reg1] = data;
                    }
                    zion::instruction::Type::R_MEM => {
                        //move to register data from cache(data)
                        //println!("type:R_MEM {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.arg1();
                        let mem_addr = instr.arg2();
                        self.registers[reg1] = self.cache.data_at(mem_addr);
                    }
                    _ => panic!("unknown instruction::Id::Mov type!"),
                }
            }
            zion::instruction::Id::Stp => {
                println!("{:?}", self);
                std::process::exit(0);
            }
            _ => panic!("unknown instruction!"),
        }
        //update instruction pointer
        //self.registers.ip += self.i_size as u8;
        self.registers.ip_update(self.i_size as u8);
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "regs:{:?}\n\nflags:{:?}\n\nio:{:?}\n\ncache:{:?}",
            &self.registers,
            &self.flags,
            &self.io,
            &self.cache
        )
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            registers: Registers::new(),
            flags: Flags::new(),
            cache: Cache::new(),
            io: Io::new(),
            i_size: 4,
        }
    }
}
