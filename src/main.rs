pub mod bus;
use std::{fs::File, io::Read};

pub use bus::{Bus, MockMem};

pub mod cpu;
pub use cpu::Cpu;

fn main() {
    let test_rom = "roms/test.bin";
    let mut fp = match File::open(test_rom) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file \"{test_rom}\": ({e})");
            return;
        }
    };
    let mut rom_buffer = Vec::new();
    let nbytes = match fp.read_to_end(&mut rom_buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Could not read file \"{test_rom}\": ({e})");
            return;
        }
    };
    println!("Read {nbytes:#x} bytes from \"{test_rom}\".");

    let mut cpu = Cpu::new();
    let mut bus = MockMem::new();
    bus.load_buffer(&rom_buffer);
    cpu.cycle(&mut bus);
}
