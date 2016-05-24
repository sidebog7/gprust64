#[derive(Default)]
pub struct Rsp;

impl Rsp {
    pub fn read_status_reg(&self) -> u32 {
        0xff
    }
}
