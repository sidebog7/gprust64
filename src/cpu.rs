use super::bus;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

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

    bus: bus::Bus,
}

impl Cpu {
    pub fn new(bus: bus::Bus) -> Cpu {
        Cpu {
            reg_gprs: [0; NUM_GPREG],
            reg_fprs: [0; NUM_FPREG],

            reg_pc: 0,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: CP0::default(),

            bus: bus,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();
    }

    pub fn run(&mut self) {}
}

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

enum RegConfigBE {
    LittleEndian,
    BigEndian,
}

impl Default for RegConfigBE {
    fn default() -> RegConfigBE {
        RegConfigBE::BigEndian
    }
}

#[derive(Default)]
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

#[derive(Default)]
struct CP0 {
    reg_config: RegConfig,
}

impl CP0 {
    fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }
}
