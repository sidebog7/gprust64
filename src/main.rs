#![deny(trivial_casts, trivial_numeric_casts)]
extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate clap;

mod n64;
mod debugger;

use std::fs;
use std::io::Read;
use std::path::Path;
use clap::{Arg, App};

use debugger::*;

fn main() {
    let matches = App::new("GPRust64")
        .version("0.1")
        .author("Gareth Pendleton <gareth.sidebottom@gmail.com>")
        .about("Beginnings of an N64 emulator")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Starts up in debug mode"))
        .arg(Arg::with_name("PIFROM")
            .help("Sets the pif rom file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("CARTROM")
            .help("Sets the cartridge rom file to use")
            .required(true)
            .index(2))
        .get_matches();

    let pif_file_name = matches.value_of("PIFROM").unwrap();
    let rom_file_name = matches.value_of("CARTROM").unwrap();

    let pif = load_bin(pif_file_name);
    let rom = load_bin(rom_file_name);

    let mut n64 = n64::N64::new(pif, rom);
    if matches.is_present("debug") {
        let mut debugger = Debugger::new(n64);
        debugger.run();
    } else {
        loop {
            n64.run_instruction();
        }
    }
}


fn load_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
