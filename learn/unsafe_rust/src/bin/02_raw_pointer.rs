fn main() {
    let mut value: i32 = 10;
    let r1: *const i32 = &value as *const i32;
    let r2: *mut i32 = &mut value as *mut i32;  // before Rust 2024
    
    let r1: *const i32 = &raw const value;
    let r2: *mut i32 = &raw mut value;         // ✅ after Rust 2024

    unsafe {
        println!("r1: {}", *r1);
        *r2 += 5;
        assert_eq!(value, 15);
    }
}