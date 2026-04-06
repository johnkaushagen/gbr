use std::fmt::Debug;

use crate::gb::cpu::instructions::op_0x31_ld_sp_n16;

use super::Memory;

mod flags;
mod instructions;
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
    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let value = memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    pub fn execute_opcode(&mut self, opcode: u8, memory: &mut Memory) -> usize {
        match opcode {
            0x31 => op_0x31_ld_sp_n16(self, memory),
            _ => todo!("op {:#04X}", opcode),
        }
    }
}
