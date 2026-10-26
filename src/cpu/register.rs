#[derive(Default)]
pub struct RegisterFile {
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
}

impl RegisterFile {
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }
    pub fn f(&self) -> u8 {
        self.f & 0xf0
    }
    pub fn set_f(&mut self, f: u8) {
        self.f = f & 0xf0;
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
    pub fn c(&self) -> u8 {
        self.c
    }
    pub fn set_c(&mut self, c: u8) {
        self.c = c;
    }
    pub fn d(&self) -> u8 {
        self.d
    }
    pub fn set_d(&mut self, d: u8) {
        self.d = d;
    }
    pub fn e(&self) -> u8 {
        self.e
    }
    pub fn set_e(&mut self, e: u8) {
        self.e = e;
    }
    pub fn h(&self) -> u8 {
        self.h
    }
    pub fn set_h(&mut self, h: u8) {
        self.h = h;
    }
    pub fn l(&self) -> u8 {
        self.l
    }
    pub fn set_l(&mut self, l: u8) {
        self.l = l;
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a(), self.f()])
    }
    pub fn set_af(&mut self, af: u16) {
        let [a, f] = af.to_be_bytes();
        self.set_a(a);
        self.set_f(a);
    }
    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b(), self.c()])
    }
    pub fn set_bc(&mut self, bc: u16) {
        let [b, c] = bc.to_be_bytes();
        self.set_b(b);
        self.set_c(c);
    }
    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d(), self.e()])
    }
    pub fn set_de(&mut self, de: u16) {
        let [d, e] = de.to_be_bytes();
        self.set_b(d);
        self.set_c(e);
    }
    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h(), self.l()])
    }
    pub fn set_hl(&mut self, hl: u16) {
        let [h, l] = hl.to_be_bytes();
        self.set_b(h);
        self.set_c(l);
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }
    pub fn inc_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }
    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }
    pub fn inc_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }
    pub fn dec_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }
}
