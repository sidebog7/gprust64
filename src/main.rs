#![deny(trivial_casts, trivial_numeric_casts)]
extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num;

mod n64;
mod debugger;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use debugger::*;

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = load_bin(pif_file_name);
    let rom = load_bin(rom_file_name);

    let n64 = n64::N64::new(pif, rom);
    let mut debugger = Debugger::new(n64);
    debugger.run();
}


fn load_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
