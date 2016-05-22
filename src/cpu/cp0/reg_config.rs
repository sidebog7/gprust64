
#[derive(Default, Debug)]
pub struct RegConfig {
    ep: EP,
    be: BE,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = EP::D;
        self.be = BE::BigEndian;
    }
}

#[derive(Debug)]
enum EP {
    D,
    DxxDxx,
    RFU,
}

impl Default for EP {
    fn default() -> EP {
        EP::D
    }
}

#[derive(Debug)]
enum BE {
    LittleEndian,
    BigEndian,
}

impl Default for BE {
    fn default() -> BE {
        BE::BigEndian
    }
}
