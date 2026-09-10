use crate::{bus::Bus, cpu::registers::Register};

pub mod flags;
pub use flags::Flags;

pub mod registers;
pub use registers::RegisterFile;

pub struct Cpu {
    registers: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
        }
    }

    pub fn cycle_instruction<B: Bus>(&mut self, bus: &mut B) {
        let opcode = self.fetch_byte(bus);
        match opcode {
            _ => todo!("op {opcode:#04x}"),
        }
    }

    pub fn fetch_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.registers.pc);
        self.registers.pc.increment();
        byte
    }
}
