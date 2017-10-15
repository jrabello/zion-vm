use std;

pub struct Cache {
    code: [u8; 0xff],
    data: [u8; 0xff],
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            code: [0; 0xff],
            data: [0; 0xff],
        }
    }

    pub fn store_code(&mut self, program_lines: &str) {
        //copies program to cache memory(code)
        if program_lines.len() > self.code.len() {
            panic!("your program is too big to fit in cache memory!");
        }
        self.code[..program_lines.len()].copy_from_slice(&program_lines.as_bytes()[..]);
    }

    pub fn get_code_at(&self, idx: usize) -> u8 {
        //gets a byte from cache memory(code)
        self.code[idx]
    }

    pub fn get_data_at(&self, mem_addr: u8) -> u8 {
        //gets data from cache memory(data)
        self.data[mem_addr as usize]
    }

    pub fn set_data_at(&mut self, mem_addr: u8, data: u8){
        self.data[mem_addr as usize] = data;
    }
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //formatting code
        write!(f, "\n\tcode:[ ");
        for &byte in &self.code[..] {
            write!(f, "{:X} ", byte).expect("Unable to write");
        }
        //formatting data
        write!(f, "]\n\tdata:[ ");
        for &byte in &self.data[..] {
            write!(f, "{:X} ", byte).expect("Unable to write");
        }
        write!(f, r"]")
    }
}
