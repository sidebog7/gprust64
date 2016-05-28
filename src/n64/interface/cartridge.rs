use byteorder::{BigEndian, ByteOrder};

const CART_ROM_HEADER_START: u32 = 0x0;
const CART_ROM_HEADER_END: u32 = 0x3f;

const CART_RAMROM_BOOTSTRAP_START: u32 = 0x040;
const CART_RAMROM_BOOTSTRAP_END: u32 = 0xB6F;

const CART_RAMROM_FONTDATA_START: u32 = 0xB70;
const CART_RAMROM_FONTDATA_END: u32 = 0xFEF;

const CRC_START: usize = 0x40;
const CRC_END: usize = 0x1000 - 0x40;
const CRC_ALECK_END: usize = 0xC00 - 0x40;

pub struct Cartridge {
    rom: Box<[u8]>,
    crc_fix: u32,
}

impl Cartridge {
    pub fn new(cartrom: Box<[u8]>) -> Cartridge {
        let crc = calc_crc(&cartrom[CRC_START..], CRC_END);
        let crc_aleck = calc_crc(&cartrom[CRC_START..], CRC_ALECK_END);
        let fix = 0;
        Cartridge {
            rom: cartrom,
            crc_fix: fix,
        }
    }



    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            CART_ROM_HEADER_START...CART_ROM_HEADER_END => self.read_cart_rom_header(addr),
            CART_RAMROM_BOOTSTRAP_START...CART_RAMROM_BOOTSTRAP_END => {
                self.read_cart_ramrom_bootstrap(addr)
            }
            CART_RAMROM_FONTDATA_START...CART_RAMROM_FONTDATA_END => {
                self.read_cart_ramrom_fontdata(addr)
            }
            _ => {
                // TODO: ??
                println!("Read Cart {:#x}", addr);
                0
            }
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            _ => {
                panic!("Cannot write to register in Cartridge {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn read_cart_rom_header(&self, addr: u32) -> u32 {
        println!("READROM {:#x}", addr);
        BigEndian::read_u32(&self.rom[(addr) as usize..])
        // self.rom[addr as usize] as u32
    }

    fn read_cart_ramrom_bootstrap(&self, addr: u32) -> u32 {
        println!("READRAMROM {:#x}", addr);
        BigEndian::read_u32(&self.rom[(addr) as usize..])
    }

    fn read_cart_ramrom_fontdata(&self, addr: u32) -> u32 {
        println!("READFONTDATA {:#x}", addr);
        BigEndian::read_u32(&self.rom[(addr) as usize..])
    }
}

fn calc_crc(rom: &[u8], size: usize) -> u32 {
    let mut table: [u32; 256] = [0; 256];
    let mut c: u32;
    for n in 0..256 {
        c = n;
        for _ in 0..8 {
            if c & 1 == 1 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c = c >> 1;
            }
        }

        table[n as usize] = c;
    }

    c = 0 ^ 0xFFFFFFFF;

    for n in 0..size {
        let p = if (n & 1) == 1 {
            n - 1
        } else {
            n + 1
        };
        c = table[((c ^ rom[p] as u32) & 0xFF) as usize] ^ (c >> 8);
    }

    c ^ 0xFFFFFFFF
}
