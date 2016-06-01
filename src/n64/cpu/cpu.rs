use std::fmt;
use std::collections::VecDeque;

use super::super::bus;
use super::registers::Registers;
use super::registers::RegistersUsed;
use super::registers::RegisterValues;
use super::cp0::CP0;
use super::instruction::Instruction;
// use super::instruction::INSTRUCTION_SIZE;
use super::opcode::Opcode;
use super::opcode::OpcodeSpecial;
use super::opcode::OpcodeRegimm;

const PIPELINE_LENGTH: usize = 5;
const PIF_ROM_START: u64 = 0xffff_ffff_bfc0_0000;

#[derive(PartialEq, Debug)]
enum PipelineStage {
    IF(u8),
    RF(u8),
    EX(u8),
    DC(u8),
    WB(u8),
    COMPLETE(u8),
}

#[derive(Debug)]
struct Pipeline {
    stage: PipelineStage,
    stalled: bool,
    instruction: Option<Instruction>,
    opcode: Option<Opcode>,
    opcode_special: Option<OpcodeSpecial>,
    opcode_regimm: Option<OpcodeRegimm>,
    result: Option<u64>,
    reg_used: Option<RegistersUsed>,
    reg_values: Option<RegisterValues>,
}

impl Pipeline {
    fn new() -> Pipeline {
        Pipeline {
            stage: PipelineStage::IF(1),
            stalled: false,
            instruction: None,
            opcode: None,
            opcode_special: None,
            opcode_regimm: None,
            result: None,
            reg_used: None,
            reg_values: None,
        }
    }

    fn run_cycle(&mut self, reg: &mut Registers, bus: &mut bus::Bus) {
        if !self.stalled {
            match self.stage {
                PipelineStage::IF(phase) => {
                    match phase {
                        1 => {
                            self.if_stage_phase1();
                        }
                        2 => {
                            self.if_stage_phase2(*reg, bus);
                        }
                        _ => {}
                    }
                }
                PipelineStage::RF(phase) => {
                    match phase {
                        1 => {
                            self.rf_stage_phase1();
                        }
                        2 => {
                            self.rf_stage_phase2(*reg);
                        }
                        _ => unreachable!(),
                    }
                }
                PipelineStage::EX(phase) => {
                    match phase {
                        1 => {
                            self.ex_stage_phase1();
                        }
                        2 => {
                            self.ex_stage_phase2();
                        }
                        _ => unreachable!(),
                    }
                }
                PipelineStage::DC(phase) => {
                    match phase {
                        1 => {
                            self.dc_stage_phase1();
                        }
                        2 => {
                            self.dc_stage_phase2();
                        }
                        _ => unreachable!(),
                    }
                }
                PipelineStage::WB(phase) => {
                    match phase {
                        1 => {
                            self.wb_stage_phase1(reg);
                        }
                        2 => {
                            self.wb_stage_phase2();
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
            self.next_stage();
        }
    }

    fn if_stage_phase1(&mut self) {
        // Phase 1
    }

    fn if_stage_phase2(&mut self, reg: Registers, bus: &mut bus::Bus) {
        // Phase 2
        self.instruction = Some(Instruction(read_word(bus, reg.reg_pc)));
    }

    fn rf_stage_phase1(&self) {
        // Phase 1
        // Cache check, could be a miss
    }

    fn rf_stage_phase2(&mut self, reg: Registers) {
        match self.instruction {
            Some(instr) => {
                // Phase 2
                // Register File Read
                let required_regs = instr.get_required_registers();
                println!("REG {:?}", required_regs);
                let reg_values = RegisterValues::new(&required_regs, reg);
                println!("VAL {:?}", reg_values);
                self.reg_used = Some(required_regs);
                self.reg_values = Some(reg_values);
                // Instruction Decode
                let opcode = instr.opcode();

                self.opcode = Some(opcode);
                match opcode {
                    Opcode::SPECIAL => {
                        self.opcode_special = Some(instr.opcode_special());
                    }
                    Opcode::REGIMM => {
                        self.opcode_regimm = Some(instr.opcode_regimm());
                    }
                    _ => {}
                }
                // VAddr Calc


            }
            None => {}
        }
    }

    fn ex_stage_phase1(&self) {
        match self.instruction {
            Some(instr) => {
                let reg_values = self.reg_values.unwrap();

                match self.opcode {
                    Some(opcode) => {
                        match opcode {
                            Opcode::SPECIAL => {
                                self.opcode_special.unwrap().ex_phase1(reg_values);
                            }
                            Opcode::REGIMM => {
                                self.opcode_regimm.unwrap().ex_phase1(reg_values);
                            }
                            _ => {
                                let imm_value = instr.immediate();
                                opcode.ex_phase1(reg_values, imm_value);
                            }
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    fn ex_stage_phase2(&mut self) {
        match self.instruction {
            Some(instr) => {
                match self.opcode {
                    Some(opcode) => {
                        let reg_values = self.reg_values.unwrap();
                        match opcode {
                            Opcode::SPECIAL => {
                                self.opcode_special.unwrap().ex_phase2(reg_values);
                            }
                            Opcode::REGIMM => {
                                self.opcode_regimm.unwrap().ex_phase2(reg_values);
                            }
                            _ => {
                                let imm_value = instr.immediate();
                                self.reg_values = Some(opcode.ex_phase2(reg_values, imm_value));
                            }
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    fn dc_stage_phase1(&self) {}

    fn dc_stage_phase2(&self) {}

    fn wb_stage_phase1(&mut self, reg: &mut Registers) {
        println!("Write {:?}", self.reg_values);

        match self.reg_values {
            Some(values) => {
                let rt = self.reg_used.unwrap().rt.unwrap();
                reg.set_gpr_val(rt, values.get_target_value());
            }
            _ => {}
        }
    }

    fn wb_stage_phase2(&self) {}

    fn next_stage(&mut self) {
        self.stage = match self.stage {
            PipelineStage::IF(phase) => {
                match phase {
                    2 => PipelineStage::RF(1),
                    _ => PipelineStage::IF(phase + 1),
                }
            }
            PipelineStage::RF(phase) => {
                match phase {
                    2 => PipelineStage::EX(1),
                    _ => PipelineStage::RF(phase + 1),
                }
            }
            PipelineStage::EX(phase) => {
                match phase {
                    2 => PipelineStage::DC(1),
                    _ => PipelineStage::EX(phase + 1),
                }
            }
            PipelineStage::DC(phase) => {
                match phase {
                    2 => PipelineStage::WB(1),
                    _ => PipelineStage::DC(phase + 1),
                }
            }
            PipelineStage::WB(phase) => {
                match phase {
                    2 => PipelineStage::COMPLETE(0),
                    _ => PipelineStage::WB(phase + 1),
                }
            }
            _ => PipelineStage::COMPLETE(0),
        };
    }

    fn is_complete(&self) -> bool {
        self.stage == PipelineStage::COMPLETE(0)
    }
}

pub struct Cpu {
    reg: Registers,

    cp0: CP0,

    bus: bus::Bus,

    pipelines: VecDeque<Pipeline>,

    pcycle_phase: u8,
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

            pcycle_phase: 1,
        }
    }

    pub fn tick(&mut self) {
        if self.pcycle_phase == 1 {
            self.setup_pipeline();
        }

        for pl in self.pipelines.iter_mut() {
            pl.run_cycle(&mut self.reg, &mut self.bus);
        }

        if self.end_tick() {
            self.pipelines.pop_front();
        }

        self.pcycle_phase = {
            match self.pcycle_phase {
                2 => 1,
                n => n + 1,
            }
        };
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
            let pl = Pipeline::new();
            self.pipelines.push_back(pl);
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

fn read_word(bus: &bus::Bus, addr: u64) -> u32 {
    let paddr = vaddr_to_paddr(addr);
    bus.read_word(paddr as u32)
}

fn write_word(bus: &mut bus::Bus, addr: u64, value: u32) {
    let paddr = vaddr_to_paddr(addr);
    bus.write_word(paddr as u32, value);
}

fn vaddr_to_paddr(vaddr: u64) -> u64 {
    let addr_bit_values = (vaddr >> 29) & 0b111;
    if addr_bit_values == 0b101 {
        // kseg1
        vaddr - 0xffff_ffff_a000_0000
    } else {
        panic!("Unrecognised virtual address {:#x}", vaddr);
    }
}
