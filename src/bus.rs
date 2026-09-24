pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct MockMem {
    data: [u8; 0x10000],
}

impl MockMem {
    pub fn new() -> Self {
        Self { data: [0; 0x10000] }
    }

    pub fn load_buffer(&mut self, buf: &[u8]) -> usize {
        let n = buf.len();
        self.data[..n].copy_from_slice(buf);
        n
    }
}

impl Bus for MockMem {
    fn read(&mut self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
