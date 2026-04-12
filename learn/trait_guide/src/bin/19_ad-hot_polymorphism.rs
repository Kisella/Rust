trait Hello {
    fn hello(&self);
}

impl Hello for i32 {
    fn hello(&self) {
        println!("Hello from i32: {}", self);
    }
}

impl Hello for String {
    fn hello(&self) {
        println!("Hello from String: {}", self);
    }
}

fn main() {

}