
#[derive(Default, Debug)]
pub struct RegConfig {
    ep: EP,
    be: BE,
    k0: K0,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = EP::D;
        self.be = BE::BigEndian;
    }

    pub fn write(&mut self, data: u32) {
        self.ep = data.into();
        self.be = data.into();
        self.k0 = data.into();
    }
}

#[derive(Debug)]
enum EP {
    D,
    DxxDxx,
    RFU,
}

impl Default for EP {
    fn default() -> Self {
        EP::D
    }
}

impl From<u32> for EP {
    fn from(f: u32) -> Self {
        let epdata = (f >> 24) & 0b1111;
        match epdata {
            0 => EP::D,
            6 => EP::DxxDxx,
            _ => EP::RFU,
        }
    }
}

#[derive(Debug)]
enum BE {
    LittleEndian,
    BigEndian,
}

impl Default for BE {
    fn default() -> Self {
        BE::BigEndian
    }
}

impl From<u32> for BE {
    fn from(f: u32) -> Self {
        let bedata = (f >> 15) & 0b1;
        if bedata == 0 {
            BE::LittleEndian
        } else {
            BE::BigEndian
        }
    }
}

#[derive(Debug)]
enum K0 {
    NotUsed,
    Used,
}

impl Default for K0 {
    fn default() -> Self {
        K0::NotUsed
    }
}

impl From<u32> for K0 {
    fn from(f: u32) -> Self {
        let k0data = f & 0b111;
        match k0data {
            0b010 => K0::NotUsed,
            _ => K0::Used,
        }
    }
}
