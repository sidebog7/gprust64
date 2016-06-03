use n64::*;
use time;

pub struct Debugger {
    n64: N64,
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger { n64: n64 }
    }

    pub fn run(&mut self) {

        let timespec = time::get_time();
        println!("TIME! {}", timespec.nsec);
        // let mut i = 0;
        // loop {
        for _ in 0..20 {
            // println!("i {:?}", i);
            // i += 1;

            self.n64.run_instruction();
        }
        let timespec2 = time::get_time();
        println!("TIME! {}", timespec2.nsec);
        println!("{:?}", self.n64.cpu);
    }
}
