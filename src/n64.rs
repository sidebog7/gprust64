use super::cpu;
use super::bus;

#[derive(Debug)]
pub struct N64 {
    cpu: cpu::Cpu,
}

impl N64 {
    pub fn new(pifrom: Vec<u8>) -> N64 {
        let bus = bus::Bus::new(pifrom);
        let cpu = cpu::Cpu::new(bus);

        N64 { cpu: cpu }
    }
    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
