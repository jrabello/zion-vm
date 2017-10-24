mod zion;
use std::fs::File;
use std::io::prelude::*;

#[macro_use] extern crate enum_primitive_derive;
extern crate num_traits;

fn main() {        
    // reading compiled binary file with instructions to execute
    let mut file = File::open("program.bin").expect("Unable to open the file");
    let mut program = String::new();
    file.read_to_string(&mut program).expect(
        "Unable to read the file",
    );

    // creating cpu instance, and running it
    zion::Cpu::new().run(&program[..]);
}
