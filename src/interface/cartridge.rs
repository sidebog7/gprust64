use byteorder::{BigEndian, ByteOrder};

const CART_ROM_HEADER_START: u32 = 0x0;
const CART_ROM_HEADER_END: u32 = 0x3f;

const CART_RAMROM_BOOTSTRAP_START: u32 = 0x040;
const CART_RAMROM_BOOTSTRAP_END: u32 = 0xB6F;

const CART_RAMROM_FONTDATA_START: u32 = 0xB70;
const CART_RAMROM_FONTDATA_END: u32 = 0xFEF;

pub struct Cartridge {
    rom: Box<[u8]>,
}

impl Cartridge {
    pub fn new(cartrom: Box<[u8]>) -> Cartridge {
        Cartridge { rom: cartrom }
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
