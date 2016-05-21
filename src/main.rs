use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

#[derive(Default)]
struct Cpu {
    reg_gprs: [u64; NUM_GPREG],
    reg_fprs: [u64; NUM_FPREG],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool,

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: CP0
}

impl Cpu {
    fn new() -> Cpu {
        Cpu::default()
    }

    fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();
    }
}

#[derive(Default)]
struct CP0 {
    reg_index: u64,
    reg_random: u64,
    reg_entrylo0: u64,
    reg_entrylo1: u64,
    reg_context: u64,
    reg_pagemask: u64,
    reg_wired: u64,
    reg_badvaddr: u64,
    reg_count: u64,
    reg_entryhi: u64,
    reg_compare: u64,
    reg_status: u64,
    reg_cause: u64,
    reg_epc: u64,
    reg_prid: u64,
    reg_config: u64,
    reg_lladdr: u64,
    reg_watchlo: u64,
    reg_watchhi: u64,
    reg_xcontext: u64,
    reg_taglo: u64,
    reg_taghi: u64,
    reg_errorepc: u64
}

impl CP0 {
    fn new() -> CP0 {
        CP0::default()
    }

    fn power_on_reset(&mut self) {

    }
}

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = load_bin(pif_file_name);
    let rom = load_bin(rom_file_name);

    let mut cpu = Cpu::new();
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
