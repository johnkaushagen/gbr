const ZFLAG: u8 = 1 << 7;
const NFLAG: u8 = 1 << 6;
const HFLAG: u8 = 1 << 5;
const CFLAG: u8 = 1 << 4;

#[derive(Debug, Default, Copy, Clone)]
pub struct Flags {
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
