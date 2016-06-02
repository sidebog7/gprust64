use super::super::bus;
use super::cp0::CP0;
use super::instruction::Instruction;
use super::opcode::Opcode;
use super::opcode::OpcodeSpecial;
use super::opcode::OpcodeRegimm;
use super::opcode::OpcodeCoproc;
use super::registers::Registers;
use super::registers::RegistersUsed;
use super::virtual_address::VAddr;
use super::instruction::INSTRUCTION_SIZE;

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
pub struct Pipeline {
    stage: PipelineStage,
    stalled: bool,
    instruction: Option<Instruction>,
    opcode: Option<Opcode>,
    opcode_special: Option<OpcodeSpecial>,
    opcode_regimm: Option<OpcodeRegimm>,
    opcode_coproc: Option<OpcodeCoproc>,
    result: Option<u64>,
    pub reg_used: Option<RegistersUsed>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            stage: PipelineStage::IF(1),
            stalled: false,
            instruction: None,
            opcode: None,
            opcode_special: None,
            opcode_regimm: None,
            opcode_coproc: None,
            result: None,
            reg_used: None,
        }
    }

    pub fn run_cycle(&mut self,
                     reg: &mut Registers,
                     bus: &mut bus::Bus,
                     cp0: &mut CP0,
                     prev_reg: Option<RegistersUsed>) {
        if !self.stalled {
            match self.stage {
                PipelineStage::IF(phase) => {
                    match phase {
                        1 => {
                            self.if_stage_phase1();
                        }
                        2 => {
                            self.if_stage_phase2(reg, bus);
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

    fn if_stage_phase2(&mut self, reg: &mut Registers, bus: &mut bus::Bus) {
        // Phase 2
        self.instruction = Some(Instruction(bus.read_word(VAddr(reg.reg_pc))));
        reg.reg_pc = reg.reg_pc.wrapping_add(INSTRUCTION_SIZE);
        println!("PC: {:#x}", reg.reg_pc);
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
                let mut required_regs = instr.get_required_registers();
                required_regs.process(reg);
                println!("REG {:?}", required_regs);
                self.reg_used = Some(required_regs);
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
                    Opcode::COPROC => {
                        self.opcode_coproc = Some(instr.opcode_coproc());
                    }
                    _ => {}
                }
                println!("Store {:?},{:?},{:?},{:?}",
                         self.opcode,
                         self.opcode_special,
                         self.opcode_regimm,
                         self.opcode_coproc);
                // VAddr Calc


            }
            None => {}
        }
    }

    fn ex_stage_phase1(&mut self) {
        println!("PHASE1 {:p}", self);
        match self.instruction {
            Some(instr) => {
                match self.opcode {
                    Some(opcode) => {
                        let reg_values = self.reg_used.unwrap();
                        match opcode {
                            Opcode::SPECIAL => {
                                self.opcode_special.unwrap().ex_phase1(reg_values);
                            }
                            Opcode::REGIMM => {
                                self.opcode_regimm.unwrap().ex_phase1(reg_values);
                            }
                            Opcode::COPROC => {
                                self.opcode_coproc.unwrap().ex_phase1(reg_values);
                            }
                            _ => {
                                let imm_value = instr.immediate();
                                self.reg_used = Some(opcode.ex_phase1(reg_values, imm_value));
                            }
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    fn ex_stage_phase2(&self) {}

    fn dc_stage_phase1(&self) {}

    fn dc_stage_phase2(&self) {}

    fn wb_stage_phase1(&mut self, reg: &mut Registers) {
        println!("Write {:?}", self.reg_used);

        match self.reg_used {
            Some(values) => {
                let rt = self.reg_used.unwrap().rt.unwrap();
                match values.get_target_value() {
                    Some(v) => {
                        reg.set_gpr_val(rt, v);
                    }
                    None => {}
                }
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

    pub fn is_complete(&self) -> bool {
        self.stage == PipelineStage::COMPLETE(0)
    }
}
