use cpu::Cpu;

use cpu::{Interconnect, Interrupt, ADDRESSABLE_MEMORY};

pub struct TestInterconnect {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl TestInterconnect {
    pub fn new() -> Self {
        TestInterconnect {
            addr: [0; ADDRESSABLE_MEMORY],
        }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.write(addr + i as u16, *byte);
        }
    }
}

impl Default for TestInterconnect {
    fn default() -> Self {
        Self::new()
    }
}

impl Interconnect for TestInterconnect {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr]
    }

    fn write(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        self.addr[addr] = data;
    }

    fn tick(&mut self) -> Interrupt {
        Interrupt::None
    }
}

pub type TestCpu = Cpu<TestInterconnect>;

impl Cpu<TestInterconnect> {
    pub fn new_test() -> Self {
        let interconnect = TestInterconnect::default();
        Cpu::new(interconnect, 0x200)
    }
}
