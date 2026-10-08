use crate::{Bus, cpu::registers::Reg8};

mod registers;
use registers::RegisterFile;

#[derive(Default)]
pub struct Cpu {
    rf: RegisterFile,
}

impl Cpu {
    pub fn cycle_instruction<B: Bus>(&mut self, bus: &mut B) -> Result<usize, String> {
        let addr = self.rf.pc;
        let opcode = self.fetch_byte(bus);
        println!("{opcode:#04x} ; {addr:#06x}");

        match opcode {
            0x00 => Ok(4),
            0x01 | 0x11 | 0x21 | 0x31 => {
                let val = self.fetch_word(bus);
                match (opcode >> 4) & 0b11 {
                    0 => self.rf.set_bc(val),
                    1 => self.rf.set_de(val),
                    2 => self.rf.set_hl(val),
                    3 => self.rf.sp = val,
                    _ => unreachable!(),
                };
                Ok(12)
            }

            0x02 | 0x12 | 0x22 | 0x32 => {
                match (opcode >> 4) & 0b11 {
                    0 => bus.write(self.rf.bc(), self.rf.a),
                    1 => bus.write(self.rf.de(), self.rf.a),
                    2 => bus.write(self.rf.hl_increment(), self.rf.a),
                    3 => bus.write(self.rf.hl_decrement(), self.rf.a),
                    _ => unreachable!(),
                };
                Ok(8)
            }

            // 16-bit increment
            0x03 | 0x13 | 0x23 | 0x33 => {
                let p = (opcode >> 4) & 0b11;
                match p {
                    0 => self.rf.set_bc(self.rf.bc().wrapping_add(1)),
                    1 => self.rf.set_de(self.rf.de().wrapping_add(1)),
                    2 => self.rf.set_hl(self.rf.hl().wrapping_add(1)),
                    3 => self.rf.sp = self.rf.sp.wrapping_add(1),
                    _ => unreachable!(),
                };
                Ok(8)
            }
            0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => {
                let y = (opcode >> 3) & 0b111;
                if y == 6 {
                    let mut val = bus.read(self.rf.hl());
                    val = self.inc_byte(val);
                    bus.write(self.rf.hl(), val);
                    return Ok(12);
                }
                self.rf[y] = self.inc_byte(self.rf[y]);
                Ok(4)
            }
            0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
                let y = (opcode >> 3) & 0b111;
                if y == 6 {
                    let mut val = bus.read(self.rf.hl());
                    val = self.dec_byte(val);
                    bus.write(self.rf.hl(), val);
                    return Ok(12);
                }
                self.rf[y] = self.dec_byte(self.rf[y]);
                Ok(4)
            }
            0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e => {
                let y = (opcode >> 3) & 0b111;
                let reg = Reg8::from_index(y);
                let val = self.fetch_byte(bus);
                reg.write(&mut self.rf, bus, val);
                if y == 6 {
                    return Ok(12);
                }
                Ok(8)
            }

            0x0a | 0x1a | 0x2a | 0x3a => {
                match (opcode >> 4) & 0b11 {
                    0 => self.rf.a = bus.read(self.rf.bc()),
                    1 => self.rf.a = bus.read(self.rf.de()),
                    2 => self.rf.a = bus.read(self.rf.hl_increment()),
                    3 => self.rf.a = bus.read(self.rf.hl_decrement()),
                    _ => unreachable!(),
                };
                Ok(8)
            }

            0x17 => {
                let carry_bit = self.rf.a >> 7;
                self.rf.a = (self.rf.a << 1) | (self.rf.flags.carry as u8);
                self.rf.flags.zero = false;
                self.rf.flags.subtract = false;
                self.rf.flags.half_carry = false;
                self.rf.flags.carry = carry_bit == 1;
                Ok(4)
            }

            0x20 => {
                let e8 = self.fetch_byte(bus) as i8;
                if !self.rf.flags.zero {
                    self.rf.pc = self.rf.pc.wrapping_add_signed(e8 as i16);
                    Ok(12)
                } else {
                    Ok(8)
                }
            }

            0x40..0x80 => {
                let src = opcode & 0b111;
                let dst = (opcode >> 3) & 0b111;
                if src != 6 && dst != 6 {
                    self.rf[dst] = self.rf[src];
                    return Ok(4);
                }
                if src == 6 {
                    if dst == 6 {
                        todo!("opcode {opcode:#04x} HALT");
                    } else {
                        self.rf[dst] = bus.read(self.rf.hl());
                        return Ok(8);
                    }
                } else {
                    bus.write(self.rf.hl(), self.rf[src]);
                    return Ok(8);
                }
            }

            // alu xor
            0xa8..0xb0 => {
                let z = opcode & 0b111;
                let reg = Reg8::from_index(z);
                let val = reg.read(&mut self.rf, bus);
                self.alu_xor(val);
                if z == 6 {
                    return Ok(8);
                }
                Ok(4)
            }

            0xc1 | 0xd1 | 0xe1 | 0xf1 => {
                let p = (opcode >> 4) & 0b11;
                let val = self.pop_stack(bus);
                match p {
                    0 => self.rf.set_bc(val),
                    1 => self.rf.set_de(val),
                    2 => self.rf.set_hl(val),
                    3 => self.rf.set_af(val),
                    _ => unreachable!(),
                }
                Ok(12)
            }

            0xc5 | 0xd5 | 0xe5 | 0xf5 => {
                let p = (opcode >> 4) & 0b11;
                let val = match p {
                    0 => self.rf.bc(),
                    1 => self.rf.de(),
                    2 => self.rf.hl(),
                    3 => self.rf.af(),
                    _ => unreachable!(),
                };
                self.rf.sp = self.rf.sp.wrapping_sub(1);
                bus.write(self.rf.sp, val as u8);
                self.rf.sp = self.rf.sp.wrapping_sub(1);
                bus.write(self.rf.sp, (val >> 8) as u8);
                Ok(16)
            }

            0xc9 => {
                // ret
                let pc = self.pop_stack(bus);
                self.rf.pc = pc;
                Ok(16)
            }
            0xcb => {
                let prefix_op = self.fetch_byte(bus);
                match prefix_op {
                    0x10..0x18 => {
                        // rl, r8
                        let z = prefix_op & 0b111;
                        let reg = Reg8::from_index(z);
                        let mut val = reg.read(&mut self.rf, bus);
                        let high_bit = val >> 7;
                        let carry_bit = self.rf.flags.carry as u8;
                        val <<= 1;
                        val |= carry_bit;
                        reg.write(&mut self.rf, bus, val);
                        self.rf.flags.zero = val == 0;
                        self.rf.flags.subtract = false;
                        self.rf.flags.half_carry = false;
                        self.rf.flags.carry = high_bit == 1;
                        if z == 6 {
                            return Ok(12);
                        }
                        Ok(8)
                    }
                    0x7c => {
                        let res = self.rf.h >> 7;
                        self.rf.flags.zero = res == 0;
                        self.rf.flags.subtract = false;
                        self.rf.flags.half_carry = true;
                        Ok(8)
                    }
                    _ => todo!("0xcb{prefix_op:02x}"),
                }
            }

            0xcd => {
                let a16 = self.fetch_word(bus);
                self.push_stack(bus, self.rf.pc);
                self.rf.pc = a16;
                Ok(24)
            }

            0xe0 | 0xf0 => {
                let a8 = self.fetch_byte(bus);
                if opcode == 0xe0 {
                    bus.write(0xff00 + (a8 as u16), self.rf.a);
                } else {
                    self.rf.a = bus.read(0xff00 + (a8 as u16));
                }
                Ok(12)
            }
            0xe2 | 0xf2 => {
                if opcode == 0xe2 {
                    bus.write(0xff00 + (self.rf.c as u16), self.rf.a);
                } else {
                    self.rf.a = bus.read(0xff00 + (self.rf.c as u16));
                }
                Ok(8)
            }
            _ => todo!("{opcode:#04x}"),
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

    pub fn pop_stack<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = bus.read(self.rf.sp);
        self.rf.sp = self.rf.sp.wrapping_add(1);
        let hi = bus.read(self.rf.sp);
        self.rf.sp = self.rf.sp.wrapping_add(1);
        u16::from_be_bytes([hi, lo])
    }
    pub fn push_stack<B: Bus>(&mut self, bus: &mut B, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.rf.sp = self.rf.sp.wrapping_sub(1);
        bus.write(self.rf.sp, hi);
        self.rf.sp = self.rf.sp.wrapping_sub(1);
        bus.write(self.rf.sp, lo);
    }

    // idu ops
    fn dec_byte(&mut self, val: u8) -> u8 {
        let new = val.wrapping_sub(1);
        self.rf.flags.zero = new == 0;
        self.rf.flags.subtract = true;
        self.rf.flags.half_carry = (val & 0x10) == 0x10;
        new
    }
    fn inc_byte(&mut self, val: u8) -> u8 {
        let new = val.wrapping_add(1);
        self.rf.flags.zero = new == 0;
        self.rf.flags.subtract = false;
        self.rf.flags.half_carry = (val & 0x0f) == 0x0f;
        new
    }
    // alu operations
    fn alu_xor(&mut self, val: u8) {
        self.rf.a = self.rf.a ^ val;
        self.rf.flags.zero = self.rf.a == 0;
        self.rf.flags.subtract = false;
        self.rf.flags.half_carry = false;
        self.rf.flags.carry = false;
    }
}

// debugging
fn print_opcode_debug_info(opcode: u8, addr: u16, info: String) {
    println!("{addr:#06x}: {info:<20}  ; (opcode: {opcode:#04x}");
}
