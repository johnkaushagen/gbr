mod cpu;
mod memory;

use std::{fs::File, io::Read};

use cpu::Cpu;
use memory::Memory;

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,

    cycles_elapsed: usize,
}

impl Emulator {
    pub fn cycle(&mut self) {
        let opcode = self.cpu.fetch_byte(&self.memory);
        let cycles = self.cpu.execute_opcode(opcode, &mut self.memory);
        self.cycles_elapsed += cycles;
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut buf).unwrap();
        self.memory.load_buffer(&buf);
    }
}
