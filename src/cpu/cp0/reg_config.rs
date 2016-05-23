
#[derive(Default, Debug)]
pub struct RegConfig {
    transfer_data_pattern: TransferDataPattern,
    endianness: Endianness,
    coherency_algorithm: CoherencyAlgorithm,
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.transfer_data_pattern = TransferDataPattern::D;
        self.endianness = Endianness::BigEndian;
    }
}

impl From<u32> for RegConfig {
    fn from(data: u32) -> Self {
        RegConfig {
            transfer_data_pattern: data.into(),
            endianness: data.into(),
            coherency_algorithm: data.into(),
        }
    }
}

#[derive(Debug)]
enum TransferDataPattern {
    D,
    DxxDxx,
    RFU,
}

impl Default for TransferDataPattern {
    fn default() -> Self {
        TransferDataPattern::D
    }
}

impl From<u32> for TransferDataPattern {
    fn from(f: u32) -> Self {
        let transfer_data_patterndata = (f >> 24) & 0b1111;
        match transfer_data_patterndata {
            0 => TransferDataPattern::D,
            6 => TransferDataPattern::DxxDxx,
            _ => TransferDataPattern::RFU,
        }
    }
}

#[derive(Debug)]
enum Endianness {
    LittleEndian,
    BigEndian,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::BigEndian
    }
}

impl From<u32> for Endianness {
    fn from(f: u32) -> Self {
        let endiannessdata = (f >> 15) & 0b1;
        if endiannessdata == 0 {
            Endianness::LittleEndian
        } else {
            Endianness::BigEndian
        }
    }
}

#[derive(Debug)]
enum CoherencyAlgorithm {
    NotUsed,
    Used,
}

impl Default for CoherencyAlgorithm {
    fn default() -> Self {
        CoherencyAlgorithm::NotUsed
    }
}

impl From<u32> for CoherencyAlgorithm {
    fn from(f: u32) -> Self {
        let coherency_algorithmdata = f & 0b111;
        match coherency_algorithmdata {
            0b010 => CoherencyAlgorithm::NotUsed,
            _ => CoherencyAlgorithm::Used,
        }
    }
}
