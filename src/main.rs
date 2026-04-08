mod gb;

use gb::Emulator;

fn main() {
    let mut emulator = Emulator::default();
    println!("{:#?}", emulator);
    emulator.load_rom("roms/dmg_boot.bin");
    for i in 0..1_000_000_000 {
        emulator.cycle();
    }
}
