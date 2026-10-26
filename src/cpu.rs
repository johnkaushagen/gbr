use crate::{Bus, cpu::register::RegisterFile};

pub mod register;

#[derive(Default)]
pub struct Cpu {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    rf: RegisterFile,
}

impl Cpu {
    pub fn emulate_instruction<B: Bus>(&mut self, bus: &mut B) {
        let addr = self.pc;
        let opcode = self.fetch_byte(bus);
        match opcode {
            0x04 => {
                // inc b
                self.b = self.inc_byte(self.b);
                let mnemonic = "inc b".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x05 => {
                // dec b
                self.b = self.dec_byte(self.b);
                let mnemonic = "dec b".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x06 => {
                // ld b, n8
                let n8 = self.fetch_byte(bus);
                self.b = n8;
                let mnemonic = format!("ld b, {n8:#04x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x0c => {
                // inc c
                let mnemonic = "inc c".to_string();
                self.c = self.inc_byte(self.c);
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x0d => {
                // dec c
                self.c = self.dec_byte(self.c);
                let mnemonic = "dec c".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x0e => {
                // ld c, n8
                let n8 = self.fetch_byte(bus);
                let mnemonic = format!("ld c, {n8:#04x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
                self.c = n8;
            }
            0x11 => {
                let lo = self.fetch_byte(bus);
                let hi = self.fetch_byte(bus);
                let de = u16::from_be_bytes([hi, lo]);
                [self.d, self.e] = de.to_be_bytes();
                let mnemonic = format!("ld de, {de:#06x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x13 => {
                let de = u16::from_be_bytes([self.d, self.e]).wrapping_add(1);
                [self.d, self.e] = de.to_le_bytes();
                let mnemonic = "inc de".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x17 => {
                // rla
                let carry_bit = match self.flag_c() {
                    true => 1,
                    false => 1,
                };
                let high_bit = self.a >> 7;
                self.a <<= 1;
                self.a |= carry_bit;
                self.set_flag_z(self.a == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(high_bit == 1);
                let mnemonic = "rla".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x18 => {
                let e8 = self.fetch_byte(bus) as i8;
                let jump_addr = self.pc.wrapping_add_signed(e8 as i16);
                let mnemonic = format!("jr {jump_addr:#06x}");
                self.pc = jump_addr;
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x1a => {
                let de = u16::from_be_bytes([self.d, self.e]);
                self.a = bus.read(de);
                let mnemonic = "ld a, (de)".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x20 => {
                // jr nz, e8
                let offset = self.fetch_byte(bus) as i8;
                let jump_addr = self.pc.wrapping_add_signed(offset as i16);
                let mnemonic = format!("jr nz, {jump_addr:#06x}");
                if !self.flag_z() {
                    self.pc = jump_addr;
                }
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x21 => {
                let lo = self.fetch_byte(bus);
                let hi = self.fetch_byte(bus);
                let hl = u16::from_be_bytes([hi, lo]);
                let mnemonic = format!("ld hl, {hl:#06x}");
                [self.h, self.l] = hl.to_be_bytes();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x22 => {
                let mut hl = u16::from_be_bytes([self.h, self.l]);
                bus.write(hl, self.a);
                hl = hl.wrapping_add(1);
                [self.h, self.l] = hl.to_be_bytes();
                let mnemonic = "ld (hl+), a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x23 => {
                // inc hl
                let mut hl = u16::from_be_bytes([self.h, self.l]);
                hl = hl.wrapping_add(1);
                [self.h, self.l] = hl.to_be_bytes();
                let mnemonic = "inc hl".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x28 => {
                let e8 = self.fetch_byte(bus) as i8;
                let jump_addr = self.pc.wrapping_add_signed(e8 as i16);
                if self.flag_z() {
                    self.pc = jump_addr;
                }
                let mnemonic = format!("jr z, {jump_addr:#06x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x2e => {
                let n8 = self.fetch_byte(bus);
                self.l = n8;
                let mnemonic = format!("ld l, {n8:#04x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x31 => {
                // ld sp, n16
                let lo = self.fetch_byte(bus);
                let hi = self.fetch_byte(bus);
                let sp = u16::from_be_bytes([hi, lo]);
                let mnemonic = format!("ld sp, {sp:#06x}");
                self.sp = sp;
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x32 => {
                // ld (hl-), a
                let mut hl = u16::from_be_bytes([self.h, self.l]);
                bus.write(hl, self.a);
                hl = hl.wrapping_sub(1);
                [self.h, self.l] = hl.to_be_bytes();
                let mnemonic = "ld (hl-), a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x3d => {
                self.a = self.dec_byte(self.a);
                let mnemonic = "dec a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x3e => {
                // ld a, n8
                let n8 = self.fetch_byte(bus);
                self.a = n8;
                let mnemonic = format!("ld a, {n8:#04x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x4f => {
                // ld c, a
                self.c = self.a;
                let mnemonic = "ld c, a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x57 => {
                self.d = self.a;
                let mnemonic = "ld d, a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x67 => {
                self.h = self.a;
                let mnemonic = "ld h, a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x77 => {
                let hl = u16::from_be_bytes([self.h, self.l]);
                bus.write(hl, self.a);
                let mnemonic = "ld (hl), a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0x7b => {
                self.a = self.e;
                let mnemonic = "ld a, e".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xaf => {
                // xor a, a
                self.a ^= self.a;
                self.f = 0x80;
                let mnemonic = "xor a, a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xc1 => {
                let bc = self.pop_stack(bus);
                [self.b, self.c] = bc.to_be_bytes();
                let mnemonic = "pop bc".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xc5 => {
                let bc = u16::from_be_bytes([self.b, self.c]);
                self.push_stack(bc, bus);
                let mnemonic = "push bc".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xc9 => {
                let ret_addr = self.pop_stack(bus);
                println!("{ret_addr:#06x}");
                self.pc = ret_addr;
                let mnemonic = "ret".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xcb => {
                // prefix instructions
                let prefix = self.fetch_byte(bus);
                match prefix {
                    0x11 => {
                        let carry_bit = match self.flag_c() {
                            true => 1,
                            false => 1,
                        };
                        let high_bit = self.c >> 7;
                        self.c <<= 1;
                        self.c |= carry_bit;
                        self.set_flag_z(self.c == 0);
                        self.set_flag_n(false);
                        self.set_flag_h(false);
                        self.set_flag_c(high_bit == 1);
                        let mnemonic = "rl c".to_string();
                        self.print_instruction_debug_info(opcode, addr, mnemonic);
                    }
                    0x7c => {
                        let bit = self.h >> 7;
                        if bit == 0 {
                            self.f &= 0b0001_0000;
                            self.f |= 0b1010_0000;
                        } else {
                            self.f &= 0b0001_0000;
                            self.f |= 0b0011_0000;
                        }
                        let mnemonic = "bit 7, h".to_string();
                        self.print_instruction_debug_info(opcode, addr, mnemonic);
                    }
                    _ => todo!("opcode 0xcb{prefix:02x}"),
                }
            }
            0xcd => {
                let lo = self.fetch_byte(bus);
                let hi = self.fetch_byte(bus);
                let a16 = u16::from_be_bytes([hi, lo]);
                self.push_stack(self.pc, bus);
                self.pc = a16;
                let mnemonic = format!("call {a16:#06x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xe0 => {
                let a8 = self.fetch_byte(bus);
                bus.write(0xff00 + (a8 as u16), self.a);
                let mnemonic = "ldh (a8), a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xe2 => {
                // ldh c, a
                let write_addr = 0xff00 + (self.c as u16);
                bus.write(write_addr, self.a);
                let mnemonic = "ldh (c), a".to_string();
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xea => {
                let lo = self.fetch_byte(bus);
                let hi = self.fetch_byte(bus);
                let a16 = u16::from_be_bytes([hi, lo]);
                bus.write(a16, self.a);
                let mnemonic = format!("ld ({a16:#06x}), a");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            0xfe => {
                // cp a, n8
                let n8 = self.fetch_byte(bus);
                let diff = self.a.wrapping_sub(n8);
                self.set_flag_z(diff == 0);
                self.set_flag_n(true);
                self.set_flag_h((self.a & 0xff) < (n8 & 0xff));
                self.set_flag_c(self.a < n8);
                let mnemonic = format!("cp a, {n8:#04x}");
                self.print_instruction_debug_info(opcode, addr, mnemonic);
            }
            _ => todo!("opcode {opcode:#04x}"),
        }
    }

    pub fn inc_byte(&mut self, byte: u8) -> u8 {
        self.set_flag_z(byte == 0xff);
        self.set_flag_n(false);
        self.set_flag_h((byte & 0x0f) == 0x0f);
        byte.wrapping_add(1)
    }
    pub fn dec_byte(&mut self, byte: u8) -> u8 {
        self.set_flag_z(byte == 0x01);
        self.set_flag_n(true);
        self.set_flag_h((byte & 0x0f) == 0x00);
        byte.wrapping_sub(1)
    }

    pub fn pop_stack<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let hi = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        u16::from_be_bytes([hi, lo])
    }
    pub fn push_stack<B: Bus>(&mut self, val: u16, bus: &mut B) {
        let [hi, lo] = val.to_be_bytes();
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, hi);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, lo);
    }

    pub fn fetch_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn flag_z(&self) -> bool {
        (self.f >> 7) == 1
    }
    pub fn set_flag_z(&mut self, val: bool) {
        self.f &= !(1 << 7);
        if val {
            self.f |= 1 << 7;
        }
    }
    pub fn flag_n(&self) -> bool {
        ((self.f >> 6) & 1) == 1
    }
    pub fn set_flag_n(&mut self, val: bool) {
        self.f &= !(1 << 6);
        if val {
            self.f |= 1 << 6;
        }
    }
    pub fn flag_h(&self) -> bool {
        ((self.f >> 5) & 1) == 1
    }
    pub fn set_flag_h(&mut self, val: bool) {
        self.f &= !(1 << 5);
        if val {
            self.f |= 1 << 5;
        }
    }
    pub fn flag_c(&self) -> bool {
        ((self.f >> 4) & 1) == 1
    }
    pub fn set_flag_c(&mut self, val: bool) {
        self.f &= !(1 << 4);
        if val {
            self.f |= 1 << 4;
        }
    }

    fn print_instruction_debug_info(&self, opcode: u8, addr: u16, mnemonic: String) {
        println!("{addr:#06x}: {mnemonic:<20}  ; {opcode:#04x}");
    }
}
