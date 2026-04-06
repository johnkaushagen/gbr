use std::fmt::Debug;

use super::Memory;

mod flags;
use flags::Flags;

#[derive(Debug, Default)]
pub struct Cpu {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn fetch_opcode(&mut self, memory: &Memory) -> u8 {
        let opcode = memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn execute_opcode(&mut self, opcode: u8, memory: &Memory) -> usize {
        match opcode {
            _ => todo!("op {:#04X}", opcode),
        }
    }
}
