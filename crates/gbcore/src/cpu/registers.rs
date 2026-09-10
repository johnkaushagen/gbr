use super::Flags;

pub struct RegisterFile {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::new(),
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }
}
