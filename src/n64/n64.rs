use super::cpu;
use super::bus;

#[derive(Debug)]
pub struct N64 {
    cpu: cpu::Cpu,
}

impl N64 {
    pub fn new(pifrom: Box<[u8]>, cartrom: Box<[u8]>) -> N64 {
        let bus = bus::Bus::new(pifrom, cartrom);
        let cpu = cpu::Cpu::new(bus);

        N64 { cpu: cpu }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_and_inc();
    }
}
