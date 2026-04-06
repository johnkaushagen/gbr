use super::Cpu;
use crate::gb::Memory;

pub fn op_0x31_ld_sp_n16(cpu: &mut Cpu, memory: &mut Memory) -> usize {
    let lo = cpu.fetch_byte(memory);
    let hi = cpu.fetch_byte(memory);
    cpu.sp = u16::from_le_bytes([lo, hi]);
    12
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
