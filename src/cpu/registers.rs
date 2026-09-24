#[derive(Default)]
pub struct RegisterFile {
    pub a: u8,
    pub flags: Flags,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub w: u8,
    pub z: u8,
}

impl RegisterFile {
    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }
    pub fn set_hl(&mut self, hl: u16) {
        [self.h, self.l] = hl.to_be_bytes();
    }
    pub fn wz(&self) -> u16 {
        u16::from_be_bytes([self.w, self.z])
    }
}

#[derive(Default)]
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
