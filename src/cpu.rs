const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

#[derive(Default, Debug)]
pub struct Cpu {
    reg_gprs: [u64; NUM_GPREG],
    reg_fprs: [u64; NUM_FPREG],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool,

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: CP0,
}

impl Cpu {
    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();
    }
}

#[derive(Debug)]
enum RegConfigEP {
    D,
    DxxDxx,
    RFU,
}

impl Default for RegConfigEP {
    fn default() -> RegConfigEP {
        RegConfigEP::D
    }
}

#[derive(Debug)]
enum RegConfigBE {
    LittleEndian,
    BigEndian,
}

impl Default for RegConfigBE {
    fn default() -> RegConfigBE {
        RegConfigBE::BigEndian
    }
}

#[derive(Default, Debug)]
struct RegConfig {
    reg_configep: RegConfigEP,
    reg_configbe: RegConfigBE,
}

impl RegConfig {
    fn power_on_reset(&mut self) {
        self.reg_configep = RegConfigEP::D;
        self.reg_configbe = RegConfigBE::BigEndian;
    }
}

#[derive(Default, Debug)]
struct CP0 {
    reg_config: RegConfig,
}

impl CP0 {
    fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }
}
