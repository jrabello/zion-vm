mod zion;
use std::fs::File;
use std::io::prelude::*;

fn main() {    
    //mov a, 0x10
    //mov b, 0x11
    //let program_lines =  "\x13\x03\x00\x10\x13\x03\x01\x11";
    
    //reading compiled file with instructions to execute
    let mut file = File::open("program.bin").expect("Unable to open the file");
    let mut program = String::new();
    file.read_to_string(&mut program).expect("Unable to read the file");

    //creating cpu instance, and running it        
    let mut cpu = zion::Cpu::new();
    cpu.init(&program[..]);      
}
