mod bit_ops;
mod cpu;
mod memory;

use std::{fs::File, io::Read};

use cpu::Cpu;
use memory::Memory;

const SCREEN_H: usize = 144;
const VBLANK_DURATION: usize = 10;
const SCANLINE_DURATION: usize = 456;

#[derive(Debug)]
pub struct Ppu {
    scanline: usize,
    dots: usize,
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            scanline: 0,
            dots: 0,
        }
    }
}

impl Ppu {
    fn cycle(&mut self, memory: &mut Memory) {
        self.dots += 4;
        if self.dots == SCANLINE_DURATION {
            self.scanline += 1;
            self.dots = 0;
        }
        if self.scanline == (SCREEN_H + VBLANK_DURATION) {
            println!("Frame finished");
            self.scanline = 0;
        }
        memory.write(0xFF44, self.scanline as u8);
    }
}

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    ppu: Ppu,

    cycles_elapsed: usize,
}

impl Emulator {
    pub fn cycle(&mut self) {
        let opcode = self.cpu.fetch_byte(&self.memory);
        let cycles = self.cpu.execute_opcode(opcode, &mut self.memory);
        self.ppu.cycle(&mut self.memory);
        self.cycles_elapsed += cycles;
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut buf).unwrap();
        self.memory.load_buffer(&buf);
    }
}
