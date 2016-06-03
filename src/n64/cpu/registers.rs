use std::fmt;
use super::opcode::Type;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

#[derive(Debug, Copy, Clone)]
pub struct RegistersUsed {
    rtype: Type,
    pub rt: Option<usize>,
    pub rt_val: Option<u64>,

    pub rs: Option<usize>,
    pub rs_val: Option<u64>,

    pub rd: Option<usize>,
    pub rd_val: Option<u64>,
}

impl RegistersUsed {
    pub fn itype(rt: usize, rs: usize) -> RegistersUsed {
        RegistersUsed {
            rtype: Type::ITYPE,
            rt: Some(rt),
            rs: Some(rs),
            rd: None,
            rt_val: None,
            rs_val: None,
            rd_val: None,
        }
    }

    pub fn rtype(rt: usize, rs: usize, rd: usize) -> RegistersUsed {
        RegistersUsed {
            rtype: Type::RTYPE,
            rt: Some(rt),
            rs: Some(rs),
            rd: Some(rd),
            rt_val: None,
            rs_val: None,
            rd_val: None,
        }
    }

    pub fn rtypecp(rt: usize, rd: usize) -> RegistersUsed {
        RegistersUsed {
            rtype: Type::RTYPECP,
            rt: Some(rt),
            rs: None,
            rd: Some(rd),
            rt_val: None,
            rs_val: None,
            rd_val: None,
        }
    }

    pub fn jtype() -> RegistersUsed {
        RegistersUsed {
            rtype: Type::JTYPE,
            rt: None,
            rs: None,
            rd: None,
            rt_val: None,
            rs_val: None,
            rd_val: None,
        }
    }

    pub fn process(&mut self, reg: Registers) {
        match self.rt {
            Some(r) => self.rt_val = Some(reg.get_gpr_val(r)),
            _ => {}
        }
        match self.rs {
            Some(r) => self.rs_val = Some(reg.get_gpr_val(r)),
            _ => {}
        }
        match self.rd {
            Some(r) => self.rd_val = Some(reg.get_gpr_val(r)),
            _ => {}
        }
    }

    pub fn process_forwarding(&mut self, regs: RegistersUsed) {
        match regs.get_output_register() {
            Some(reg) => {
                match self.rt {
                    Some(reg2) => {
                        if reg == reg2 {

                            self.rt_val = regs.get_output_value();
                        }
                    }
                    _ => {}
                }
                match self.rs {
                    Some(reg2) => {
                        if reg == reg2 {
                            self.rs_val = regs.get_output_value();
                            // println!("SETTING RS {:?}", self.rs_val);
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    pub fn with_output(&self, value: u64) -> RegistersUsed {
        RegistersUsed {
            rtype: self.rtype,
            rt: self.rt,
            rt_val: match self.rtype {
                Type::ITYPE => Some(value),
                _ => self.rt_val,
            },
            rs: self.rs,
            rs_val: self.rs_val,
            rd: self.rd,
            rd_val: match self.rtype {
                Type::RTYPE | Type::RTYPECP => Some(value),
                _ => self.rd_val,
            },
        }
    }

    fn get_output_register(&self) -> Option<usize> {
        match self.rtype {
            Type::RTYPE | Type::RTYPECP => self.rd,
            Type::ITYPE => self.rt,
            _ => None,
        }
    }

    fn get_output_value(&self) -> Option<u64> {
        match self.rtype {
            Type::RTYPE | Type::RTYPECP => self.rd_val,
            Type::ITYPE => self.rt_val,
            _ => None,
        }
    }

    pub fn get_target_value(&self) -> Option<u64> {
        self.rt_val
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct RegisterValues {
//     rt: Option<u64>,
//     rs: Option<u64>,
//     rd: Option<u64>,
// }
//
//
// impl RegisterValues {
//     pub fn new(used: &RegistersUsed, reg: Registers) -> RegisterValues {
//         RegisterValues {
//             rt: match used.rt {
//                 Some(r) => Some(reg.get_gpr_val(r)),
//                 _ => None,
//             },
//             rs: match used.rs {
//                 Some(r) => Some(reg.get_gpr_val(r)),
//                 _ => None,
//             },
//             rd: match used.rs {
//                 Some(r) => Some(reg.get_gpr_val(r)),
//                 _ => None,
//             },
//         }
//     }
//
//     pub fn target(value: u64) -> RegisterValues {
//         RegisterValues {
//             rt: Some(value),
//             rs: None,
//             rd: None,
//         }
//     }
//
//     pub fn get_target_value(&self) -> u64 {
//         self.rt.unwrap_or_else(|| panic!("Unknown target value {:?}", self))
//     }
// }

#[derive(Default, Clone, Copy)]
pub struct Registers {
    reg_gprs: [u64; NUM_GPREG],
    reg_fprs: [f64; NUM_FPREG],

    pub reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool,

    reg_fcr0: u32,
    reg_fcr31: u32,
}

impl Registers {
    pub fn get_gpr_val(&self, reg: usize) -> u64 {
        self.reg_gprs[reg]
    }

    pub fn set_gpr_val(&mut self, reg: usize, value: u64) {
        self.reg_gprs[reg] = value;
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPREG] =
            ["r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5",
             "t6", "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1",
             "gp", "sp", "s8", "ra"];

        try!(write!(f, "\nCPU General Purpose Registers:"));
        for reg_num in 0..NUM_GPREG {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(f,
                        "{reg_name}/gpr{num:02}: {value:#018X} ",
                        num = reg_num,
                        reg_name = REG_NAMES[reg_num],
                        value = self.reg_gprs[reg_num]));
        }

        try!(write!(f, "\n\nCPU Floating Point Registers:"));
        for reg_num in 0..NUM_FPREG {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(f,
                "fpr{num:02}: {value:21} ",
                num = reg_num,
                value = self.reg_fprs[reg_num],));
        }

        try!(writeln!(f, "\n\nCPU Special Registers:"));
        writeln!(f,
                 "\
            reg_pc: {:#018X}\nreg_hi: {:#018X}\nreg_lo: {:#018X}\nreg_llbit: \
                  {}\nreg_fcr0:  {:#010X}\nreg_fcr31: {:#010X}\n",
                 self.reg_pc,
                 self.reg_hi,
                 self.reg_lo,
                 self.reg_llbit,
                 self.reg_fcr0,
                 self.reg_fcr31)
    }
}
