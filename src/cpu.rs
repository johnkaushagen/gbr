mod registers;
use registers::RegisterFile;

#[derive(Default)]
pub struct Cpu {
    reg: RegisterFile,
}

impl Cpu {
    pub fn cycle(&mut self, bus: &mut [u8; 0x10000]) {
        let opcode = self.fetch_byte(bus);
        match opcode {
            _ => todo!("opcode {opcode:#04x}"),
        }
    }

    pub fn fetch_byte(&mut self, bus: &mut [u8; 0x10000]) -> u8 {
        let result = bus[self.reg.pc as usize];
        self.reg.pc = self.reg.pc.wrapping_add(1);
        result
    }
}
