use std::{cmp::min, fmt::Debug};

pub struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn load_buffer(&mut self, buf: &[u8]) {
        let n = min(self.data.len(), buf.len());
        self.data[..n].copy_from_slice(&buf[..n]);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self { data: [0; 0x10000] }
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memory: 64 KiB").finish()
    }
}
