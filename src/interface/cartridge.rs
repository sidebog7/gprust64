use byteorder::{BigEndian, ByteOrder};

const CART_ROM_HEADER_START: u32 = 0x0;
const CART_ROM_HEADER_END: u32 = 0x3f;

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
            _ => panic!("Unknown address in Cartridge {:#x}", addr),
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
        BigEndian::read_u32(&self.rom[(addr - CART_ROM_HEADER_START) as usize..])
        // self.rom[addr as usize] as u32
    }
}
