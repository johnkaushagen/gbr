use super::Cpu;
use crate::gb::{Memory, bit_ops::u16_to_bytes};

pub fn op_0x04_inc_b(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.b = cpu.b.wrapping_add(1);
    cpu.flags.zero = cpu.b == 0;
    cpu.flags.subtract = false;
    cpu.flags.halfcarry = cpu.b & 0x0F == 0;
    4
}

pub fn op_0x05_dec_b(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.flags.zero = cpu.b == 0;
    cpu.flags.subtract = true;
    cpu.flags.halfcarry = (cpu.b & 0x0F) == 0x0F;
    4
}

pub fn op_0x06_ld_b_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.b = cpu.fetch_byte(memory);
    8
}

pub fn op_0x0c_inc_c(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.c = cpu.c.wrapping_add(1);
    cpu.flags.zero = cpu.c == 0;
    cpu.flags.subtract = false;
    cpu.flags.halfcarry = cpu.c & 0x0F == 0;
    4
}

pub fn op_0x0d_dec_c(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.flags.zero = cpu.c == 0;
    cpu.flags.subtract = true;
    cpu.flags.halfcarry = (cpu.c & 0x0F) == 0x0F;
    4
}

pub fn op_0x0e_ld_c_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.c = cpu.fetch_byte(memory);
    8
}

pub fn op_0x11_ld_de_n16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.e = cpu.fetch_byte(memory);
    cpu.d = cpu.fetch_byte(memory);
    12
}

pub fn op_0x13_inc_de(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.set_de(cpu.de().wrapping_add(1));
    8
}
pub fn op_0x17_rla(cpu: &mut Cpu, _: &mut Memory) -> usize {
    let old_carry = cpu.flags.carry as u8;
    let high_bit = (cpu.a >> 7) == 1;
    cpu.a <<= 1;
    cpu.a |= old_carry;
    cpu.flags.carry = high_bit;
    cpu.flags.zero = false;
    4
}

pub fn op_0x18_jr_e8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let e8 = cpu.fetch_byte(memory) as i8;
    cpu.pc = cpu.pc.wrapping_add_signed(e8 as i16);
    12
}

pub fn op_0x1a_ld_a_de_indirect(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.a = memory.read(cpu.de());
    8
}

pub fn op_0x1e_ld_e_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.e = cpu.fetch_byte(memory);
    8
}

pub fn op_0x20_jr_nz_e8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let e8 = cpu.fetch_byte(memory) as i8;
    if !cpu.flags.zero {
        cpu.pc = cpu.pc.wrapping_add_signed(e8 as i16);
        12
    } else {
        8
    }
}

pub fn op_0x21_ld_hl_n16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.l = cpu.fetch_byte(memory);
    cpu.h = cpu.fetch_byte(memory);
    12
}

pub fn op_0x22_ld_hli_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    memory.write(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    8
}

pub fn op_0x23_inc_hl(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.set_hl(cpu.hl().wrapping_add(1));
    8
}

pub fn op_0x28_jr_z_e8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let e8 = cpu.fetch_byte(memory) as i8;
    if cpu.flags.zero {
        cpu.pc = cpu.pc.wrapping_add_signed(e8 as i16);
        12
    } else {
        8
    }
}

pub fn op_0x2e_ld_l_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.l = cpu.fetch_byte(memory);
    8
}

pub fn op_0x31_ld_sp_n16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let lo = cpu.fetch_byte(memory);
    let hi = cpu.fetch_byte(memory);
    cpu.sp = u16::from_le_bytes([lo, hi]);
    12
}

pub fn op_0x32_ld_hld_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    memory.write(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    8
}

pub fn op_0x3d_dec_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.flags.zero = cpu.a == 0;
    cpu.flags.subtract = true;
    cpu.flags.halfcarry = cpu.a & 0x0F == 0x0F;
    4
}

pub fn op_0x3e_ld_a_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.a = cpu.fetch_byte(memory);
    8
}

pub fn op_0x4f_ld_c_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.c = cpu.a;
    4
}

pub fn op_0x57_ld_d_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.d = cpu.a;
    4
}

pub fn op_0x67_ld_h_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.h = cpu.a;
    4
}

pub fn op_0x77_ld_hl_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let hl = u16::from_le_bytes([cpu.l, cpu.h]);
    memory.write(hl, cpu.a);
    8
}

pub fn op_0x7b_ld_a_e(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.a = cpu.e;
    4
}
pub fn op_0xaf_xor_a_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.a = 0;
    cpu.flags.zero = true;
    cpu.flags.subtract = false;
    cpu.flags.halfcarry = false;
    cpu.flags.carry = false;
    4
}

pub fn op_0xc1_pop_bc(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.c = memory.read(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.b = memory.read(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(1);
    12
}

pub fn op_0xc5_push_bc(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, cpu.b);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, cpu.c);
    16
}

pub fn op_0xc9_ret(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let pc_lsb = memory.read(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(1);
    let pc_msb = memory.read(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.pc = u16::from_le_bytes([pc_lsb, pc_msb]);
    16
}

pub fn op_0xcd_call_a16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let lo = cpu.fetch_byte(memory);
    let hi = cpu.fetch_byte(memory);
    let (pc_msb, pc_lsb) = u16_to_bytes(cpu.pc);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, pc_msb);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, pc_lsb);
    cpu.pc = u16::from_le_bytes([lo, hi]);
    24
}

pub fn op_0xe0_ldh_a8_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let a8 = cpu.fetch_byte(memory);
    let addr: u16 = 0xFF00 + a8 as u16;
    memory.write(addr, cpu.a);
    12
}

pub fn op_0xe2_ldh_c_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let addr: u16 = 0xFF00 + cpu.c as u16;
    memory.write(addr, cpu.a);
    8
}

pub fn op_0xea_ld_a16_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let a16_lsb = cpu.fetch_byte(memory);
    let a16_msb = cpu.fetch_byte(memory);
    let a16 = u16::from_le_bytes([a16_lsb, a16_msb]);
    memory.write(a16, cpu.a);
    16
}

pub fn op_0xf0_ldh_a_a8_indirect(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let a8 = cpu.fetch_byte(memory);
    let addr = 0xFF00 + a8 as u16;
    cpu.a = memory.read(addr);
    8
}

pub fn op_0xfe_cp_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let n8 = cpu.fetch_byte(memory);
    let res = cpu.a.wrapping_sub(n8);
    cpu.flags.zero = res == 0;
    cpu.flags.subtract = true;
    cpu.flags.halfcarry = (n8 & 0x0F) > (cpu.a & 0x0F);
    cpu.flags.carry = n8 > cpu.a;
    8
}

pub fn prefix_0x11_rl_c(cpu: &mut Cpu, _: &mut Memory) -> usize {
    let old_carry = cpu.flags.carry;
    let bit7 = cpu.c >> 7;
    cpu.c <<= 1;
    cpu.flags.carry = if bit7 == 1 { true } else { false };
    cpu.c |= old_carry as u8;
    cpu.flags.zero = cpu.c == 0;
    8
}

pub fn prefix_0x7c_bit_7_h(cpu: &mut Cpu, _: &mut Memory) -> usize {
    let is_set = (cpu.h >> 7) & 1 != 1;
    cpu.flags.zero = is_set;
    cpu.flags.subtract = false;
    cpu.flags.halfcarry = true;
    8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_op_0x0c_inc_c() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();
        op_0x0c_inc_c(&mut cpu, &mut memory);
        assert_eq!(cpu.c, 0x01);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.subtract);
        assert!(!cpu.flags.halfcarry);
        cpu.c = 0x0F;
        op_0x0c_inc_c(&mut cpu, &mut memory);
        assert_eq!(cpu.c, 0x10);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.subtract);
        assert!(cpu.flags.halfcarry);
        cpu.c = 0xFF;
        op_0x0c_inc_c(&mut cpu, &mut memory);
        assert_eq!(cpu.c, 0x00);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.subtract);
        assert!(cpu.flags.halfcarry);
    }

    #[test]
    fn test_op_0x31_ld_sp_n16() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();
        memory.write(0x0000, 0xCD);
        memory.write(0x0001, 0xAB);
        let cycles = op_0x31_ld_sp_n16(&mut cpu, &mut memory);
        assert_eq!(cycles, 12);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.sp, 0xABCD);
    }

    #[test]
    fn test_prefix_0x7c_bit_7_h() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();
        cpu.h = 0b10000000;
        let ticks = prefix_0x7c_bit_7_h(&mut cpu, &mut memory);
        assert_eq!(ticks, 8);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.subtract);
        assert!(cpu.flags.halfcarry);
        cpu.h = 0;
        let ticks = prefix_0x7c_bit_7_h(&mut cpu, &mut memory);
        assert_eq!(ticks, 8);
        assert!(cpu.flags.zero);
        assert!(!cpu.flags.subtract);
        assert!(cpu.flags.halfcarry);
    }
}
