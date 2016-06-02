use super::super::bus;
use super::cp0::CP0;
use super::instruction::Instruction;
use super::opcode::Opcode;
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

#[derive(Default, Debug)]
pub struct DataWrite {
    pub cp0_data_register: Option<usize>,
    pub cp0_data_to_write: u64,
}



#[derive(Debug)]
pub struct Pipeline {
    stage: PipelineStage,
    stalled: u8,
    instruction: Option<Instruction>,
    opcode: Option<Opcode>,
    pub reg_used: Option<RegistersUsed>,
    output_data: DataWrite,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            stage: PipelineStage::IF(1),
            stalled: 0,
            instruction: None,
            opcode: None,
            reg_used: None,
            output_data: DataWrite::default(),
        }
    }

    pub fn run_cycle(&mut self,
                     reg: &mut Registers,
                     bus: &mut bus::Bus,
                     cp0: &mut CP0,
                     prev_reg: Option<RegistersUsed>) {
        if self.stalled > 0 {
            self.stalled -= 1;
        } else {
            println!("RUNNING stage {:?}", self.stage);
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
                    self.check_forwarding(prev_reg);
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
                            self.wb_stage_phase1(reg, cp0);
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

    fn check_forwarding(&mut self, prev_reg: Option<RegistersUsed>) {
        match prev_reg {
            Some(regs) => {
                println!("Processing forwading {:?}", regs);
                let mut new_reg = self.reg_used.unwrap();
                new_reg.process_forwarding(regs);
                self.reg_used = Some(new_reg);
                println!("COMPLETE {:?}", self.reg_used);
            }
            None => {}
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
                // VAddr Calc


            }
            None => {}
        }
    }

    fn ex_stage_phase1(&mut self) {
        match self.instruction {
            Some(instr) => {
                match self.opcode {
                    Some(opcode) => {
                        let reg_values = self.reg_used.unwrap();
                        println!("USING {:?}", reg_values);
                        let imm_value = instr.immediate();
                        self.reg_used =
                            Some(opcode.ex_phase1(reg_values, imm_value, &mut self.output_data));
                        println!("Storing {:?}", self.reg_used);
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
                        let reg_values = self.reg_used.unwrap();

                        let imm_value = instr.immediate();
                        self.reg_used = Some(opcode.ex_phase2(reg_values, imm_value));
                        println!("Storing {:?}", self.reg_used);
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    fn dc_stage_phase1(&self) {}

    fn dc_stage_phase2(&self) {}

    fn wb_stage_phase1(&mut self, reg: &mut Registers, cp0: &mut CP0) {
        println!("Write {:?}", self.reg_used);

        match self.output_data.cp0_data_register {
            Some(reg) => {
                cp0.write_reg(reg, self.output_data.cp0_data_to_write);
            }
            None => {}
        }
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
