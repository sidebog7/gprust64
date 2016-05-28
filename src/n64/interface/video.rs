const VI_V_CURRENT_REG: u32 = 0x10;
const VI_INTR_REG: u32 = 0x0c;
const VI_H_START_REG: u32 = 0x24;


#[derive(Default, Debug)]
pub struct Video {
    intr_half_line: u32,
    horizontal_video_start: u16,
    horizontal_video_end: u16,
    current_vertical_line: u16,
}

impl Video {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            VI_INTR_REG => self.read_halfline(),
            VI_H_START_REG => self.read_h_video(),
            VI_V_CURRENT_REG => self.read_current_vertical_line() as u32,
            _ => panic!("Unknown address in Video {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            VI_INTR_REG => self.write_halfline(value),
            VI_H_START_REG => self.write_h_video(value),
            VI_V_CURRENT_REG => self.write_current_vertical_line(value),
            _ => {
                panic!("Cannot write to register in Video {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn read_halfline(&self) -> u32 {
        self.intr_half_line
    }

    fn write_halfline(&mut self, value: u32) {
        self.intr_half_line = value & 0x3ff;
    }

    fn read_h_video(&self) -> u32 {
        (self.horizontal_video_start as u32) << 16 | (self.horizontal_video_end as u32)
    }

    fn write_h_video(&mut self, value: u32) {
        self.horizontal_video_start = (value >> 16 & 0x3ff) as u16;
        self.horizontal_video_end = (value & 0x3ff) as u16;
    }

    fn read_current_vertical_line(&self) -> u16 {
        self.current_vertical_line & 0x3ff
    }

    fn write_current_vertical_line(&mut self, value: u32) {
        self.current_vertical_line = (value & 0x3ff) as u16;
        // TODO clear interrupt line
    }
}
