use std;

pub struct Io {
    in_: u8,
    out: u8,
}
impl Io {
    pub fn new() -> Io {
        Io { in_: 0, out: 0 }
    }
    pub fn get_in(&self) -> u8{
        self.in_
    }
    pub fn set_out(&mut self, value: u8){
        self.out = value;
    }
}
impl std::fmt::Debug for Io {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r"
            in:  {:X}h {:b}b
            out: {:X}h {:b}b",
            &self.in_,
            &self.in_,
            &self.out,
            &self.out
        )
        //self.code[..].fmt(formatter) + " " + self.data[..].fmt(formatter)
    }
}
