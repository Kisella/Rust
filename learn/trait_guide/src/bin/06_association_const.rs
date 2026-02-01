trait Limit {
    const MAX: u32;
}

struct Counter;
impl Limit for Counter {
    const MAX: u32 = 100;
}

fn main() {
    
}