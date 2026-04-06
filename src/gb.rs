mod cpu;
mod memory;

use std::{fs::File, io::Read};

use cpu::Cpu;
use memory::Memory;

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn cycle(&mut self) {
        let opcode = self.cpu.fetch_opcode(&self.memory);
        println!("Fetched op {:#04X}", opcode);
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut buf).unwrap();
        self.memory.load_buffer(&buf);
    }
}
