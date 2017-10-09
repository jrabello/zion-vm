use std;
use zion;
use zion::Instruction;
use zion::Registers;
use zion::Flags;
use zion::Cache;
use zion::Io;

pub struct Cpu {
    io: Io,
    registers: Registers,
    flags: Flags,
    cache: Cache,    
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            flags: Flags::new(),
            cache: Cache::new(),
            io: Io::new(),            
        }
    }

    pub fn init(&mut self, program_lines: &str) {
        //copying program to cache(code)
        self.cache.store_code(program_lines);

        //cpu pipeline
        loop {            
            let byte = self.fetch(0);
            let current_instruction = self.decode(byte);
            self.execute(current_instruction);
        }
    }

    fn fetch(&self, offset: usize) -> u8 {
        //get current byte from cache mem(code)
        let base = self.registers.ip() as usize;
        self.cache.get_code_at(base + offset)
    }

    fn decode(&mut self, byte: u8) -> Instruction {
        //decode instruction
        let mut instr = Instruction::new(byte);
        match instr.get_id() {
            zion::instruction::Id::Mov => {                
                instr.set_type(self.fetch(1));
                instr.set_arg1(self.fetch(2));
                instr.set_arg2(self.fetch(3));
                instr.set_size(4);
            },
            zion::instruction::Id::Stp => {
                instr.set_size(1);
            },
            _ => panic!("unkown instruction!"),
        }
        //update instruction pointer
        self.registers.ip_update(instr.get_size() as u8);
        instr
    }

    fn execute(&mut self, instr: Instruction) {
        //execute instruction
        match instr.get_id() {
            zion::instruction::Id::Mov => {
                //println!("{}", "mov!");
                match instr.get_type() {
                    zion::instruction::Type::R_R => {
                        //move register to register
                        //println!("type:R_R {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.get_arg1();
                        let reg2 = instr.get_arg2();
                        self.registers[reg1] = self.registers[reg2];
                    }
                    zion::instruction::Type::R_IMM => {
                        //move to register IMM value
                        //println!("type:R_IMM {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.get_arg1();
                        let data = instr.get_arg2();
                        self.registers[reg1] = data;
                    }
                    zion::instruction::Type::R_MEM => {
                        //move to register data from cache(data)
                        //println!("type:R_MEM {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.get_arg1();
                        let mem_addr = instr.get_arg2();
                        self.registers[reg1] = self.cache.get_data_at(mem_addr);
                    }
                    _ => panic!("unknown instruction::Id::Mov type!"),
                }
            }
            zion::instruction::Id::Stp => {
                println!("{:?}", self);
                std::process::exit(0);
            }
            _ => {
                println!("{:?}", instr.get_id());
                panic!("unknown instruction!");
            }
        }
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
        }
    }
}