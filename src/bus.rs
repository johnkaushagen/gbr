pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

impl Bus for [u8; 0x10000] {
    fn read(&mut self, addr: u16) -> u8 {
        self[addr as usize]
    }
    fn write(&mut self, addr: u16, value: u8) {
        self[addr as usize] = value;
    }
}
