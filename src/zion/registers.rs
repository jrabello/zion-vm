use std;
use std::ops::{Index, IndexMut};

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    ip: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            ip: 0,
        }
    }

    pub fn ip_update(&mut self, value: u8) {
        self.ip += value;
    }

    pub fn ip(&self) -> u8 {
        self.ip
    }

    pub fn set_value(&mut self, reg: u8, value: u8){
        self[reg] = value;
    }

    pub fn get_value(&self, reg: u8) -> u8{
        self[reg]
    }

    fn move_rr(&mut self, reg1: u8, reg2: u8) {
        self[reg1] = self[reg2];
    }
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            ip: 0,
        }
    }    
}

impl Index<u8> for Registers {
    type Output = u8;
    fn index(&self, idx: u8) -> &u8 {
        // '
        match idx {
            0 => &self.a,                
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            _ => panic!("unknown register: {}", idx),
        }
    }
}

impl IndexMut<u8> for Registers {
    //type Output = u8;
    fn index_mut(&mut self, idx: u8) -> &mut u8 {
        match idx {
            0 => &mut self.a,                
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.e,
            5 => &mut self.f,
            _ => panic!("unknown register: {}", idx),
        }
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,r" 
            a:  {:X}h {:b}b
            b:  {:X}h {:b}b
            c:  {:X}h {:b}b
            d:  {:X}h {:b}b
            e:  {:X}h {:b}b
            f:  {:X}h {:b}b
            ip: {:X}h {:b}b", 
            &self.a, &self.a,                 
            &self.b, &self.b,                
            &self.c,&self.c,
            &self.d,&self.d,
            &self.e,&self.e,
            &self.f,&self.f,
            &self.ip,&self.ip,)
        //self.code[..].fmt(formatter) + " " + self.data[..].fmt(formatter)
    }
}
//}
