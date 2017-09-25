use std;

pub struct Flags {
    zero: u8,
    carry: u8,
}

impl Flags {
    pub fn new() -> Flags {
        Flags { zero: 0, carry: 0 }
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r"
            zero:  {} 
            carry: {}",
            &self.zero,
            &self.carry
        )
    }
}
