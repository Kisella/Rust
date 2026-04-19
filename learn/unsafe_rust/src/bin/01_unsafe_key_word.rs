unsafe fn unsafe_fn() {}
unsafe trait UnsafeTrait {}

fn main() {
    unsafe {
        unsafe_fn();
    }
}