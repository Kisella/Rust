mod math {
    pub mod basic {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}
fn main() {
    println!("2 + 3 = {}", math::basic::add(2, 3));
    println!("2 * 3 = {}", math::basic::multiply(2, 3))
}