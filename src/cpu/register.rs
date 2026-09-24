pub struct RegisterFile {
    r: [u8; 0x8],
    pc_lo: u8,
    pc_hi: u8,
    sp_lo: u8,
    sp_hi: u8,
    flags: Flags,
}

impl RegisterFile {
    const B_REG_INDEX: usize = 0;
    const C_REG_INDEX: usize = 0;
    const D_REG_INDEX: usize = 0;
    const E_REG_INDEX: usize = 0;
    const H_REG_INDEX: usize = 0;
    const L_REG_INDEX: usize = 0;
    const HL_REG_INDEX: usize = 0;
    const A_REG_INDEX: usize = 0;

    pub fn new() -> Self {
        Self {
            r: [0; 0x8],
            pc_hi: 0,
            pc_lo: 0,
            sp_hi: 0,
            sp_lo: 0,
            flags: Flags::new(),
        }
    }

    pub fn pc(&self) -> u16 {
        u16::from_be_bytes([self.pc_hi, self.pc_lo])
    }
    pub fn inc_pc(&mut self) {
        self.set_pc(self.pc().wrapping_add(1));
    }
    pub fn set_pc(&mut self, pc: u16) {
        [self.pc_hi, self.pc_lo] = pc.to_le_bytes();
    }
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    const ZBIT: usize = 7;
    const NBIT: usize = 6;
    const HBIT: usize = 5;
    const CBIT: usize = 4;

    const ZMASK: u8 = 1 << Self::ZBIT;
    const NMASK: u8 = 1 << Self::NBIT;
    const HMASK: u8 = 1 << Self::HBIT;
    const CMASK: u8 = 1 << Self::CBIT;

    pub fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn as_u8(&self) -> u8 {
        ((self.zero as u8) * Self::ZMASK)
            | ((self.subtract as u8) * Self::NMASK)
            | ((self.half_carry as u8) * Self::HMASK)
            | ((self.carry as u8) * Self::CMASK)
    }
}
