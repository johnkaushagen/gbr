mod gb;

use gb::Emulator;

fn main() {
    let mut emulator = Emulator::default();
    println!("{:#?}", emulator);
}
