#[derive(Default)]
pub struct Rsp;

impl Rsp {
    pub fn read_status_reg(&self) -> u32 {
        1 // TODO too similar to getRandomNumber() [https://xkcd.com/221/]
    }
}
