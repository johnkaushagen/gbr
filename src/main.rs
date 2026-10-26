pub mod bus;
use std::{
    fs::File,
    io::{self, Read},
};

pub use bus::Bus;
mod cpu;
use cpu::Cpu;

fn main() {
    let mut fp = match File::open("roms/test.bin") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file. {e}");
            return;
        }
    };
    let mut rom_buffer = Vec::from([0u8; 0x10000]);
    let nbytes = match fp.read(&mut rom_buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Could not read file. {e}");
            return;
        }
    };
    let mut cpu = Cpu::default();
    let mut input = String::new();
    while input != "q".to_string() {
        cpu.emulate_instruction(&mut rom_buffer);
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("failed to read user input");
    }
}
