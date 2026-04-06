const ZFLAG: u8 = 1 << 7;
const NFLAG: u8 = 1 << 6;
const HFLAG: u8 = 1 << 5;
const CFLAG: u8 = 1 << 4;

#[derive(Debug, Default, Copy, Clone)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub halfcarry: bool,
    pub carry: bool,
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        (value.zero as u8 * ZFLAG)
            | (value.subtract as u8 * NFLAG)
            | (value.halfcarry as u8 * HFLAG)
            | (value.carry as u8 * CFLAG)
    }
}
