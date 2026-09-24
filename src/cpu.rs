use crate::Bus;

mod register;
use register::RegisterFile;

pub struct Cpu {
    registers: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
        }
    }

    pub fn cycle<B: Bus>(&mut self, bus: &mut B) -> usize {
        println!("cycle");
        0
    }
}
