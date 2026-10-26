pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

impl Bus for Vec<u8> {
    fn read(&mut self, address: u16) -> u8 {
        self[address as usize]
    }
    fn write(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }
}
