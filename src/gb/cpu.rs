use std::fmt::Debug;

use super::Memory;

use super::bit_ops::u16_to_bytes;
mod flags;
mod instructions;

use flags::Flags;
use instructions::*;

#[derive(Debug, Default)]
pub struct Cpu {
    a: u8,
    flags: Flags,
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
    pub fn de(&self) -> u16 {
        u16::from_le_bytes([self.e, self.d])
    }

    pub fn set_de(&mut self, value: u16) {
        (self.d, self.e) = u16_to_bytes(value);
    }

    pub fn hl(&self) -> u16 {
        u16::from_le_bytes([self.l, self.h])
    }

    pub fn set_hl(&mut self, value: u16) {
        (self.h, self.l) = u16_to_bytes(value);
    }

    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let value = memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    pub fn execute_opcode(&mut self, opcode: u8, memory: &mut Memory) -> usize {
        match opcode {
            0x06 => op_0x06_ld_b_n8(self, memory),
            0x0C => op_0x0c_inc_c(self, memory),
            0x0E => op_0x0e_ld_c_n8(self, memory),
            0x11 => op_0x11_ld_de_n16(self, memory),
            0x1A => op_0x1a_ld_a_de_indirect(self, memory),
            0x20 => op_0x20_jr_nz_e8(self, memory),
            0x21 => op_0x21_ld_hl_n16(self, memory),
            0x31 => op_0x31_ld_sp_n16(self, memory),
            0x32 => op_0x32_ld_hld_indirect_a(self, memory),
            0x3E => op_0x3e_ld_a_n8(self, memory),
            0x4F => op_0x4f_ld_c_a(self, memory),
            0x77 => op_0x77_ld_hl_indirect_a(self, memory),
            0xAF => op_0xaf_xor_a_a(self, memory),
            0xCB => self.execute_prefix(memory),
            0xCD => op_0xcd_call_a16(self, memory),
            0xE0 => op_0xe0_ldh_a8_indirect_a(self, memory),
            0xE2 => op_0xe2_ldh_c_indirect_a(self, memory),
            _ => todo!("op {:#04X}", opcode),
        }
    }

    pub fn execute_prefix(&mut self, memory: &mut Memory) -> usize {
        let opcode = self.fetch_byte(memory);
        match opcode {
            0x7C => prefix_0x7c_bit_7_h(self, memory),
            _ => todo!("op 0xCB{:02X}", opcode),
        }
    }
}
