pub mod bus;
use std::{fs::File, io::Read, io::Write};

pub use bus::Bus;
mod cpu;
use cpu::Cpu;

fn main() {
    let mut fp = match File::open("roms/dmg_boot.bin") {
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
    let mut stdout = std::io::stdout();
    while input != "q".to_string() {
        writeln!(stdout, "{}", cpu.gbdoctor_string(&mut rom_buffer));
        cpu.emulate_instruction(&mut rom_buffer);
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("failed to read user input");
    }
}
