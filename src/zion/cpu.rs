use std;
use zion;
use zion::{Instruction, Registers, Flags, Cache, Io};


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

    pub fn run(&mut self, program_lines: &str) {
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
            zion::instruction::Id::In => {
                instr.set_type(self.fetch(1));
                instr.set_op1(self.fetch(2));
            }
            zion::instruction::Id::Out => {
                instr.set_type(self.fetch(1));
                instr.set_op1(self.fetch(2));
            }
            zion::instruction::Id::Add => {
                instr.set_type(self.fetch(1));
                instr.set_op1(self.fetch(2));
                instr.set_op2(self.fetch(3));
            }
            zion::instruction::Id::Mov => {
                instr.set_type(self.fetch(1));
                instr.set_op1(self.fetch(2));
                instr.set_op2(self.fetch(3));
            }
            zion::instruction::Id::Stp => {
                //println!("{:?}", self);
                //std::process::exit(0);
            }
            _ => panic!("unknown instruction!"),
        }
        //update instruction pointer
        self.registers.ip_update(instr.get_size() as u8);
        instr
    }

    fn execute(&mut self, instr: Instruction) {
        //execute instruction
        match instr.get_id() {
            zion::instruction::Id::In => {
                match instr.get_type() {
                    zion::instruction::Type::R => {
                        let reg = instr.get_op1();
                        self.registers.set_value(reg, self.io.get_in());
                    }
                    zion::instruction::Type::MEM => {
                        let mem_addr = instr.get_op1();
                        self.cache.set_data_at(mem_addr, self.io.get_in());
                    }
                    _ => panic!("unknown Id::In type!"),
                }
            }
            zion::instruction::Id::Out => {
                match instr.get_type() {
                    zion::instruction::Type::R => {
                        let reg = instr.get_op1();
                        self.io.set_out(self.registers.get_value(reg));
                    }
                    zion::instruction::Type::MEM => {
                        let mem_addr = instr.get_op1();
                        self.io.set_out(self.cache.get_data_at(mem_addr));
                    }
                    _ => panic!("unknown Id::Out type!"),
                }
            }
            zion::instruction::Id::Mov => {
                println!("mov :{:?} {} {}", instr.get_type(), instr.get_op1(), instr.get_op2());
                match instr.get_type() {
                    zion::instruction::Type::R_R => {
                        //move register to register
                        //println!("type:R_R {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.get_op1();
                        let reg2 = instr.get_op2();
                        let reg_value = self.registers.get_value(reg2);
                        self.registers.set_value(reg1, reg_value);
                    }
                    zion::instruction::Type::R_IMM => {
                        //move IMM value to register
                        println!("type:R_IMM {} {}:", instr.get_op1(), instr.get_op2());
                        let reg1 = instr.get_op1();
                        let imm = instr.get_op2();
                        self.registers.set_value(reg1, imm);
                    }
                    zion::instruction::Type::R_MEM => {
                        //move data from cache(data) to register
                        //println!("type:R_MEM {} {}:", instr.arg1(), instr.arg2());
                        let reg1 = instr.get_op1();
                        let mem_addr = instr.get_op2();
                        self.registers.set_value(reg1, self.cache.get_data_at(mem_addr));
                    }
                    zion::instruction::Type::MEM_R => {
                        let mem_addr = instr.get_op1();
                        let reg = instr.get_op2();
                        self.cache.set_data_at(
                            mem_addr,
                            self.registers.get_value(reg),
                        );
                    }
                    _ => panic!("unknown Id::Mov type!"),
                }
            }
            zion::instruction::Id::Add => {
                match instr.get_type() {
                    zion::instruction::Type::R_R => {
                        let reg1 = instr.get_op1();
                        let reg1_value = self.registers.get_value(reg1);
                        let reg2_value = self.registers.get_value(instr.get_op2());
                        self.registers.set_value(reg1, reg1_value + reg2_value);
                    }
                    zion::instruction::Type::R_IMM => {
                        let reg = instr.get_op1();
                        let imm = instr.get_op2();
                        let value = self.registers.get_value(reg);
                        self.registers.set_value(reg, value + imm);
                    }
                    zion::instruction::Type::R_MEM => {
                        let reg = instr.get_op1();
                        let reg_value = self.registers.get_value(reg);
                        let mem_addr = instr.get_op2();
                        let mem_addr_value = self.cache.get_data_at(mem_addr);
                        self.registers.set_value(reg, reg_value + mem_addr_value);
                    }
                    _ => panic!("unknown Id::Add type!"),
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