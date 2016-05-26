
#[derive(Default, Debug)]
pub struct RegConfig {
    transfer_data_pattern: TransferDataPattern,
    endianness: Endianness,
    cu: bool,
    kseg0_cache_bits: [bool; 3],
}

impl From<u32> for RegConfig {
    fn from(data: u32) -> Self {
        RegConfig {
            transfer_data_pattern: data.into(),
            endianness: data.into(),
            cu: (data >> 3) & 0b1 != 0,
            kseg0_cache_bits: [(data & (1 << 0)) != 0,
                               (data & (1 << 1)) != 0,
                               (data & (1 << 2)) != 0],
        }
    }
}

impl RegConfig {
    fn get_kseg0_coherency_algorithm(&self) -> CoherencyAlgorithm {
        let mut cache_value = 0;
        if self.kseg0_cache_bits[0] {
            cache_value |= 1 << 0;
        }
        if self.kseg0_cache_bits[1] {
            cache_value |= 1 << 1;
        }
        if self.kseg0_cache_bits[2] {
            cache_value |= 1 << 2;
        }
        cache_value.into()
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
        match endiannessdata {
            0 => Endianness::LittleEndian,
            1 => Endianness::BigEndian,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum CoherencyAlgorithm {
    NotUsed(u8),
    Used(u8),
}

impl Default for CoherencyAlgorithm {
    fn default() -> Self {
        CoherencyAlgorithm::NotUsed(0b000)
    }
}

impl From<u32> for CoherencyAlgorithm {
    fn from(f: u32) -> Self {
        let coherency_algorithmdata = (f & 0b111) as u8;
        if coherency_algorithmdata == 0b010 {
            CoherencyAlgorithm::NotUsed(coherency_algorithmdata)
        } else {
            CoherencyAlgorithm::Used(coherency_algorithmdata)
        }

    }
}
