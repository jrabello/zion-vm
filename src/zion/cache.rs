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

    pub fn store_code(&mut self, program_lines: &str){
        //copies program to cache memory(code)
        if program_lines.len() > 0xff {
            panic!("your program is too bit to fit in cache(code) memory!");
        }
        self.code[..program_lines.len()].copy_from_slice(&program_lines.as_bytes()[..]);
    
    }

    pub fn get_code_at(&self, idx: usize) -> u8 {
        //gets a line from cache memory(code)
        self.code[idx]
    }

    pub fn get_data_at(&self, mem_addr: u8) -> u8 {
        //gets data from cache memory(data)
        self.data[mem_addr as usize]
    }
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;
        let mut scode = String::new();
        let mut sdata = String::new();

        for &byte in &self.code[..] {
            write!(&mut scode, "{:X} ", byte).expect("Unable to write");
        }
        for &byte in &self.data[..] {
            write!(&mut sdata, "{:X} ", byte).expect("Unable to write");
        }
        
        write!(
            f,
            r"
            code:[ {}]
            data:[ {}]",
            scode,
            sdata
        )
    }
}
