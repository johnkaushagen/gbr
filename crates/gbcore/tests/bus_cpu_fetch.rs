use gbcore::Cpu;
use gbcore::bus::Bus;

pub struct MockBus {
    pub mem: [u8; 0x10000],
}
impl MockBus {
    pub fn new() -> Self {
        Self { mem: [0; 0x10000] }
    }
}

impl Bus for MockBus {
    fn read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    fn write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
}

#[test]
fn test_bus_cpu_fetch_byte() {
    let mut cpu = Cpu::new();
    let mut mock_bus = MockBus::new();
    mock_bus.mem[0] = 0xAB;
    mock_bus.mem[1] = 0xCD;
    mock_bus.mem[2] = 0xEF;
    let mut byte = cpu.fetch_byte(&mut mock_bus);
    assert_eq!(byte, mock_bus.mem[0]);
    byte = cpu.fetch_byte(&mut mock_bus);
    assert_eq!(byte, mock_bus.mem[1]);
    byte = cpu.fetch_byte(&mut mock_bus);
    assert_eq!(byte, mock_bus.mem[2]);
}

#[test]
fn test_cpu_instruction_cycle() {
    let mut cpu = Cpu::new();
    let mut bus = MockBus::new();
    bus.mem[0] = 0;
    cpu.cycle_instruction(&mut bus);
}
