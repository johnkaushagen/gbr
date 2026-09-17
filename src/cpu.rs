#[derive(Default)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Cpu {
    pub fn cycle(&mut self, _bus: &mut [u8; 0x10000]) {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ByteRegister {
    // i hate this name
    A,
    B,
    C,
    D,
    E,
    F, // I think this one should not be here
    H,
    L,
    HLRef,
}
impl ByteRegister {
    pub fn what_to_call_this(index: usize) -> Result<Self, String> {
        match index {
            0 => Ok(Self::B),
            1 => Ok(Self::C),
            2 => Ok(Self::D),
            3 => Ok(Self::E),
            4 => Ok(Self::H),
            5 => Ok(Self::L),
            6 => Ok(Self::HLRef),
            7 => Ok(Self::A),
            _ => Err(format!("Wrong index {index} for register table.")),
        }
    }
}
