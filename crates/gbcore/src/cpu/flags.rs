const ZERO_BIT: u8 = 7;
const SUBTRACT_BIT: u8 = 6;
const HALF_CARRY_BIT: u8 = 5;
const CARRY_BIT: u8 = 4;

const ZERO: u8 = 1 << ZERO_BIT;
const SUBTRACT: u8 = 1 << SUBTRACT_BIT;
const HALF_CARRY: u8 = 1 << HALF_CARRY_BIT;
const CARRY: u8 = 1 << CARRY_BIT;

pub struct Flags(u8);

impl Flags {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn bits(&self) -> u8 {
        self.0
    }
    pub fn zero(&self) -> bool {
        (self.0 & ZERO) != 0
    }
    pub fn subtract(&self) -> bool {
        (self.0 & SUBTRACT) != 0
    }
    pub fn half_carry(&self) -> bool {
        (self.0 & HALF_CARRY) != 0
    }
    pub fn carry(&self) -> bool {
        (self.0 & CARRY) != 0
    }
    pub fn set_zero(&mut self, zero: bool) {
        self.set_bit(ZERO_BIT, zero);
    }
    pub fn set_subtract(&mut self, subtract: bool) {
        self.set_bit(SUBTRACT_BIT, subtract);
    }
    pub fn set_half_carry(&mut self, half_carry: bool) {
        self.set_bit(HALF_CARRY_BIT, half_carry);
    }
    pub fn set_carry(&mut self, carry: bool) {
        self.set_bit(CARRY_BIT, carry)
    }

    fn set_bit(&mut self, bit: u8, val: bool) {
        self.0 = ((val as u8) << bit) | (self.0 & !(1 << bit));
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let mut f = Flags::new();
        assert!(!f.zero());
        f.set_zero(true);
        assert!(f.zero());
        assert_eq!(f.bits(), ZERO);
        f.set_zero(false);
        assert!(!f.zero());
        assert_eq!(f.bits(), 0);
    }

    #[test]
    fn test_subtract() {
        let mut f = Flags::new();
        assert!(!f.subtract());
        f.set_subtract(true);
        assert!(f.subtract());
        assert_eq!(f.bits(), SUBTRACT);
        f.set_subtract(false);
        assert!(!f.subtract());
        assert_eq!(f.bits(), 0);
    }

    #[test]
    fn test_half_carry() {
        let mut f = Flags::new();
        assert!(!f.half_carry());
        f.set_half_carry(true);
        assert!(f.half_carry());
        assert_eq!(f.bits(), HALF_CARRY);
        f.set_half_carry(false);
        assert!(!f.half_carry());
        assert_eq!(f.bits(), 0);
    }

    #[test]
    fn test_carry() {
        let mut f = Flags::new();
        assert!(!f.carry());
        f.set_carry(true);
        assert!(f.carry());
        assert_eq!(f.bits(), CARRY);
        f.set_carry(false);
        assert!(!f.carry());
        assert_eq!(f.bits(), 0);
    }
}
