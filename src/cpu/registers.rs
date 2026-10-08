use std::ops::{Index, IndexMut};

use crate::Bus;

#[derive(Default)]
pub struct RegisterFile {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a: u8,
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
}

impl RegisterFile {
    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }
    pub fn set_bc(&mut self, val: u16) {
        [self.b, self.c] = val.to_be_bytes();
    }
    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }
    pub fn set_de(&mut self, val: u16) {
        [self.d, self.e] = val.to_be_bytes();
    }
    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }
    pub fn set_hl(&mut self, hl: u16) {
        [self.h, self.l] = hl.to_be_bytes();
    }
    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.flags.as_u8()])
    }
    pub fn set_af(&mut self, val: u16) {
        let [a, f] = val.to_be_bytes();
        self.a = a;
        self.flags = f.into();
    }

    pub fn hl_increment(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl.wrapping_add(1));
        hl
    }
    pub fn hl_decrement(&mut self) -> u16 {
        let old_hl = self.hl();
        self.set_hl(old_hl.wrapping_sub(1));
        old_hl
    }
}

impl Index<u8> for RegisterFile {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.b,
            1 => &self.c,
            2 => &self.d,
            3 => &self.e,
            4 => &self.h,
            5 => &self.l,
            6 => todo!("this is the (hl) index. need to figure out how to deal with it"),
            7 => &self.a,
            _ => panic!("invalid index"),
        }
    }
}

impl IndexMut<u8> for RegisterFile {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.b,
            1 => &mut self.c,
            2 => &mut self.d,
            3 => &mut self.e,
            4 => &mut self.h,
            5 => &mut self.l,
            6 => todo!("this is the (hl) index. need to figure out how to deal with it"),
            7 => &mut self.a,
            _ => panic!("invalid index"),
        }
    }
}

#[derive(Default)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}
impl Flags {
    pub fn as_u8(&self) -> u8 {
        ((self.zero as u8) << 7)
            | ((self.subtract as u8) << 6)
            | ((self.half_carry as u8) << 5)
            | ((self.carry as u8) << 4)
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            zero: (value >> 7) & 1 != 0,
            subtract: (value >> 6) & 1 != 0,
            half_carry: (value >> 5) & 1 != 0,
            carry: (value >> 4) & 1 != 1,
        }
    }
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        ((value.zero as u8) << 7)
            | ((value.subtract as u8) << 6)
            | ((value.half_carry as u8) << 5)
            | ((value.carry as u8) << 4)
    }
}

pub enum Reg8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HLread,
    A,
}

impl Reg8 {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::HLread,
            7 => Self::A,
            _ => panic!("Invalid index {index} for Reg::from_index"),
        }
    }
    pub fn read<B: Bus>(&self, rf: &mut RegisterFile, bus: &mut B) -> u8 {
        match self {
            Self::B => rf.b,
            Self::C => rf.c,
            Self::D => rf.d,
            Self::E => rf.e,
            Self::H => rf.h,
            Self::L => rf.l,
            Self::HLread => bus.read(rf.hl()),
            Self::A => rf.a,
        }
    }
    pub fn write<B: Bus>(&self, rf: &mut RegisterFile, bus: &mut B, val: u8) {
        match self {
            Self::B => rf.b = val,
            Self::C => rf.c = val,
            Self::D => rf.d = val,
            Self::E => rf.e = val,
            Self::H => rf.h = val,
            Self::L => rf.l = val,
            Self::HLread => bus.write(rf.hl(), val),
            Self::A => rf.a = val,
        }
    }
}
