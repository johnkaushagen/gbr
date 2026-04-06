use std::fmt::Debug;

use super::Memory;

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

// Flags

const ZFLAG: u8 = 1 << 7;
const NFLAG: u8 = 1 << 6;
const HFLAG: u8 = 1 << 5;
const CFLAG: u8 = 1 << 4;

#[derive(Debug, Default, Copy, Clone)]
struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        (value.z as u8 * ZFLAG)
            | (value.n as u8 * NFLAG)
            | (value.h as u8 * HFLAG)
            | (value.c as u8 * CFLAG)
    }
}

trait Register {}
