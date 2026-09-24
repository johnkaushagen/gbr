use std::{env, fs::File, io::Read};
pub mod bus;
pub mod cpu;

pub use bus::Bus;
pub use cpu::Cpu;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filepath = args[1].as_str();
    let mut f = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file \"{filepath}\". ({e})");
            return;
        }
    };
    let mut buffer = [0u8; 0x10000];
    match f.read(&mut buffer) {
        Err(e) => {
            eprintln!("Could not read file \"{filepath}\". ({e})");
            return;
        }
        _ => (),
    };
    let mut cpu = Cpu::new(&mut buffer);
    loop {
        cpu.cycle(&mut buffer);
    }
}
