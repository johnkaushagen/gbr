mod gb;

use gb::Emulator;

fn main() {
    let mut emulator = Emulator::default();
    println!("{:#?}", emulator);
    emulator.load_rom("roms/dmg_boot.bin");
    loop {
        emulator.cycle();
    }
}
