use std::fmt;
use std::collections::VecDeque;

use super::super::bus;
use super::registers::Registers;
use super::registers::RegistersUsed;
use super::cp0::CP0;
use super::pipeline::Pipeline;

const PIPELINE_LENGTH: usize = 5;
const PIF_ROM_START: u64 = 0xffff_ffff_bfc0_0000;

pub struct Cpu {
    reg: Registers,

    cp0: CP0,

    bus: bus::Bus,

    pipelines: VecDeque<Pipeline>,

    pcycle_phase: u64,
}

impl Cpu {
    pub fn new(bus: bus::Bus) -> Cpu {
        let mut reg = Registers::default();
        reg.reg_pc = PIF_ROM_START;

        Cpu {
            reg: reg,

            cp0: CP0::default(),

            bus: bus,

            pipelines: VecDeque::with_capacity(5),

            pcycle_phase: 0,
        }
    }

    pub fn tick(&mut self) {
        // println!("Tick {}", self.pcycle_phase);
        if self.pcycle_phase % 2 == 0 {
            self.setup_pipeline();
        }

        let mut last_used: Option<RegistersUsed> = None;
        for pl in self.pipelines.iter_mut() {

            pl.run_cycle(&mut self.reg, &mut self.bus, &mut self.cp0, last_used);

            last_used = pl.reg_used;
        }

        if self.end_tick() {
            self.pipelines.pop_front();
        }

        self.pcycle_phase += 1;
    }

    pub fn end_tick(&mut self) -> bool {

        let front = self.pipelines.front();

        match front {
            Some(pl) => pl.is_complete(),
            None => false,
        }

    }
    pub fn setup_pipeline(&mut self) {
        if self.pipelines.len() < PIPELINE_LENGTH {
            self.pipelines.push_back(Pipeline::new());
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // try!(writeln!(f, "{:#?}", self.reg));
        // try!(writeln!(f, "{:#?}", self.cp0));
        try!(writeln!(f, "{:#?}", self.pipelines));
        writeln!(f, "{:#?}", self.bus)
    }
}
