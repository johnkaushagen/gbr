use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HLRef,
    A,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    AF,
    PC,
    SP,
}

impl Display for Reg8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B => "b".fmt(f),
            Self::C => "c".fmt(f),
            Self::D => "d".fmt(f),
            Self::E => "e".fmt(f),
            Self::H => "h".fmt(f),
            Self::L => "l".fmt(f),
            Self::HLRef => "(hl)".fmt(f),
            Self::A => "a".fmt(f),
        }
    }
}

impl Display for Reg16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BC => "bc".fmt(f),
            Self::DE => "de".fmt(f),
            Self::HL => "hl".fmt(f),
            Self::AF => "af".fmt(f),
            Self::PC => "pc".fmt(f),
            Self::SP => "sp".fmt(f),
        }
    }
}

#[derive(Default)]
pub struct RegisterFile {
    pub r: [u8; 8],
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
}

impl RegisterFile {
    pub fn get(&mut self, register: Reg8) -> u8 {
        self.r[register as usize]
    }
    pub fn set(&mut self, register: Reg8, value: u8) {
        self.r[register as usize] = value;
    }

    pub fn get_word(&mut self, register: Reg16) -> u16 {
        match register {
            Reg16::BC => u16::from_be_bytes([self.get(Reg8::B), self.get(Reg8::C)]),
            Reg16::DE => u16::from_be_bytes([self.get(Reg8::D), self.get(Reg8::E)]),
            Reg16::HL => u16::from_be_bytes([self.get(Reg8::H), self.get(Reg8::L)]),
            Reg16::AF => u16::from_be_bytes([self.get(Reg8::A), self.flags.into()]),
            Reg16::PC => self.pc,
            Reg16::SP => self.sp,
        }
    }

    pub fn set_word(&mut self, register: Reg16, value: u16) {
        match register {
            Reg16::BC => {
                self.set(Reg8::B, (value >> 8) as u8);
                self.set(Reg8::C, value as u8);
            }
            Reg16::DE => {
                self.set(Reg8::D, (value >> 8) as u8);
                self.set(Reg8::E, value as u8);
            }
            Reg16::HL => {
                self.set(Reg8::H, (value >> 8) as u8);
                self.set(Reg8::L, value as u8);
            }
            Reg16::AF => {
                self.set(Reg8::A, (value >> 8) as u8);
                self.flags = (value as u8).into();
            }
            Reg16::PC => self.pc = value,
            Reg16::SP => self.sp = value,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
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
