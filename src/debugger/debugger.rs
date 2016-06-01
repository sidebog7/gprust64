use n64::*;

pub struct Debugger {
    n64: N64,
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger { n64: n64 }
    }

    pub fn run(&mut self) {

        // let mut i = 0;
        // loop {
        for _ in 0..10 {
            // println!("i {:?}", i);
            // i += 1;
            self.n64.run_instruction();
        }
        println!("{:?}", self.n64.cpu);
    }
}
