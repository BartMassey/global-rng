use rand::*;
use rand::rngs::SmallRng;
use ugly_global::*;

global_vars! {
    RNG: R;
}

struct R {
    gen: Box<dyn RngCore + Send + Sync>,
}

impl R {
    fn new() -> R {
        R { gen: Box::new(SmallRng::from_entropy()) }
    }
}

struct BadR {
    state: u32,
}

const WEYL_CONSTANT: u64 = 0xb5ad4eceda1ce2a9;

impl BadR {
    fn new() -> BadR {
        // println!("!");
        BadR { state: 2107866292 }
    }

    fn next_u32(&mut self) -> u32 {
        let s = self.state as u64;
        self.state = (u64::wrapping_add(WEYL_CONSTANT, s * s) >> 16) as u32;
        self.state
    }
}
    
impl RngCore for BadR {
    fn next_u32(&mut self) -> u32 {
        let r = self.next_u32();
        // println!("→u32 {}", r);
        r
    }
    fn next_u64(&mut self) -> u64 {
        let r1 = self.next_u32() as u64;
        let r2 = self.next_u32() as u64;
        // println!("→u64 {} {}", r1, r2);
        r1 << 32 | r2
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for b in dest.iter_mut() {
            *b = self.next_u32() as u8;
        }
        // println!("[u8] {:?}", dest);
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
    drop(r);
    demo();
    let tid = std::thread::spawn(|| {
        demo();
    });
    tid.join().unwrap();
}
