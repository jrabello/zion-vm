mod zion;
use std::fs::File;
use std::io::prelude::*;

#[macro_use] extern crate enum_primitive_derive;
extern crate num_traits;

fn main() {
    //mov a, 0x10
    //mov b, 0x11
    //let program_lines =  "\x13\x03\x00\x10\x13\x03\x01\x11";

    // match Thing::from_u8(1) {
    //     Some(Thing::Foo) => println!("foo"),
    //     Some(Thing::Bar) =>  println!("bar"),
    //     None => panic!("out of range"),
    // }

    //println!("{}", (Instr::mov as u8) == 0);
    //0 == 0

    //reading compiled binary file with instructions to execute
    let mut file = File::open("program.bin").expect("Unable to open the file");
    let mut program = String::new();
    file.read_to_string(&mut program).expect(
        "Unable to read the file",
    );

    //creating cpu instance, and running it
    zion::Cpu::new().run(&program[..]);
}
