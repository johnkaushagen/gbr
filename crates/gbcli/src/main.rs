use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: gbcli path/to/rom");
        return;
    }
    let rom_path = args[1].as_str();
    let mut rom = match File::open(rom_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file \"{rom_path}\": {e}");
            return;
        }
    };
    let mut buffer: Vec<u8> = Vec::new();
    let nbytes = match rom.read_to_end(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error reading file \"{rom_path}\": {e}");
            return;
        }
    };
    println!("Read {nbytes} bytes from file \"{rom_path}\"");

    let mut pc = 0;
    while pc < nbytes {
        let opcode = buffer[pc];
        println!("{opcode:#04x} \t\t // {pc:#06x}");
        pc += 1;
    }
}
