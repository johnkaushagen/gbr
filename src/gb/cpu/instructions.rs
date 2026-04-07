use super::Cpu;
use crate::gb::{Memory, bit_ops::u16_to_bytes};

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

pub fn op_0x0e_ld_c_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.c = cpu.fetch_byte(memory);
    8
}

pub fn op_0x11_ld_de_n16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.e = cpu.fetch_byte(memory);
    cpu.d = cpu.fetch_byte(memory);
    12
}

pub fn op_0x1a_ld_a_de_indirect(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.a = memory.read(cpu.de());
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
    println!(
        "PC: {:#06X} | LD HL, 0x{:02X}{:02X}",
        cpu.pc - 3,
        cpu.h,
        cpu.l
    );
    12
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

pub fn op_0x3e_ld_a_n8(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    cpu.a = cpu.fetch_byte(memory);
    8
}

pub fn op_0x4f_ld_c_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.c = cpu.a;
    4
}

pub fn op_0x77_ld_hl_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let hl = u16::from_le_bytes([cpu.l, cpu.h]);
    memory.write(hl, cpu.a);
    8
}

pub fn op_0xaf_xor_a_a(cpu: &mut Cpu, _: &mut Memory) -> usize {
    cpu.a = 0;
    cpu.flags.zero = true;
    cpu.flags.subtract = false;
    cpu.flags.halfcarry = false;
    cpu.flags.carry = false;
    4
}

pub fn op_0xcd_call_a16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let (pc_msb, pc_lsb) = u16_to_bytes(cpu.pc);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, pc_msb);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write(cpu.sp, pc_lsb);
    let lo = cpu.fetch_byte(memory);
    let hi = cpu.fetch_byte(memory);
    cpu.pc = u16::from_le_bytes([lo, hi]);
    println!("CALL {:#06X}", cpu.pc);
    24
}

pub fn op_0xe0_ldh_a8_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let a8 = cpu.fetch_byte(memory);
    let addr: u16 = 0xFF00 + a8 as u16;
    memory.write(addr, cpu.a);
    println!("PC: {:#06X} -- LDH ({:#04X}), A", cpu.pc - 2, a8);
    12
}

pub fn op_0xe2_ldh_c_indirect_a(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let addr: u16 = 0xFF00 + cpu.c as u16;
    memory.write(addr, cpu.a);
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
