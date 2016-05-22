mod n64;
mod cpu;
mod bus;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif = load_bin(pif_file_name);
    let rom = load_bin(rom_file_name);

    let mut n64 = n64::N64::new(pif);
    n64.power_on_reset();
    n64.run();


}


fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
