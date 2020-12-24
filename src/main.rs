use rand::*;
use ugly_global::*;

global_vars! {
    RNG: R;
}

struct R {
    gen: Box<dyn RngCore>,
}

impl R {
    fn new() -> R {
        R { gen: Box::new(thread_rng()) }
    }
}

struct BadR {
    state: u64,
}

impl BadR {
    fn new() -> BadR {
        BadR { state: 0 }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = (self.state + 5) % 7;
        self.state
    }
}
    
impl RngCore for BadR {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for b in dest {
            *b = self.next_u64() as u8;
        }
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn demo() {
    fetch!{ r = RNG };
    let v = r.gen.gen_range(1..=6);
    println!("{}", v);
}

fn main() {
    init!{ RNG = R::new() };
    demo();
    fetch!{ r = RNG };
    r.gen = Box::new(BadR::new());
    demo();
    /*
    let r = std::sync::Mutex::new(r);
    let tid = std::thread::spawn(move || {
        let v = r.lock().unwrap().gen.gen_range(1..=6);
        println!("{}", v);
    });
    tid.join().unwrap();
    */
}
