use crate::Bus;

mod registers;
use registers::RegisterFile;

#[derive(Default)]
struct ExecutionContext {
    pub current_opcode: u8,
    pub current_step: usize,
}

impl ExecutionContext {
    pub fn preload_opcode<B: Bus>(&mut self, bus: &mut B) {
        self.current_opcode = bus.read(0);
    }
}

pub struct Cpu {
    rf: RegisterFile,
    context: ExecutionContext,
}

impl Cpu {
    pub fn new<B: Bus>(bus: &mut B) -> Self {
        let mut context = ExecutionContext::default();
        context.preload_opcode(bus);
        Self {
            rf: RegisterFile {
                pc: 1,
                ..Default::default()
            },
            context: context,
        }
    }

    pub fn cycle<B: Bus>(&mut self, bus: &mut B) {
        match self.context.current_opcode {
            0x21 => {
                // ld hl, n16
                match self.context.current_step {
                    0 => {
                        self.rf.z = self.fetch_byte(bus);
                        println!("z <- {:#04x}", self.rf.z);
                        self.context.current_step += 1;
                        return;
                    }
                    1 => {
                        self.rf.w = self.fetch_byte(bus);
                        println!("w <- {:#04x}", self.rf.w);
                        self.context.current_step += 1;
                        return;
                    }
                    2 => {
                        self.rf.set_hl(self.rf.wz());
                        self.context.current_step = 0;
                        self.context.current_opcode = self.fetch_byte(bus);
                        println!("ld hl, {:#06x}", self.rf.hl());
                    }
                    _ => todo!("invalid step. should this be an error, or do we just crash?"),
                }
            }
            0x31 => {
                // ld sp, n16
                match self.context.current_step {
                    0 => {
                        self.rf.z = self.fetch_byte(bus);
                        println!("z <- {:#04x}", self.rf.z);
                        self.context.current_step += 1;
                        return;
                    }
                    1 => {
                        self.rf.w = self.fetch_byte(bus);
                        println!("w <- {:#04x}", self.rf.w);
                        self.context.current_step += 1;
                        return;
                    }
                    2 => {
                        self.rf.sp = self.rf.wz();
                        self.context.current_step = 0;
                        self.context.current_opcode = self.fetch_byte(bus);
                        println!("ld sp, {:#06x}", self.rf.sp)
                    }
                    _ => todo!("invalid step. should this be an error, or do we just crash?"),
                }
            }
            0xaf => {
                println!("xor a, a");
                self.rf.a ^= self.rf.a;
                self.rf.flags.zero = true;
                self.rf.flags.subtract = false;
                self.rf.flags.half_carry = false;
                self.rf.flags.carry = false;
                self.context.current_step = 0;
                self.context.current_opcode = self.fetch_byte(bus);
            }
            _ => todo!("{:#04x}", self.context.current_opcode),
        }
    }

    pub fn fetch_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = self.rf.pc;
        self.rf.pc = self.rf.pc.wrapping_add(1);
        bus.read(addr)
    }

    pub fn fetch_word<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let word_lo = self.fetch_byte(bus);
        let word_hi = self.fetch_byte(bus);
        u16::from_le_bytes([word_lo, word_hi])
    }
}
