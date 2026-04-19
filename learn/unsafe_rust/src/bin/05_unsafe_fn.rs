static mut NUM: i32 = 10;
unsafe fn change_static_num() {
    NUM += 1;
}

// pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
//     String { vec: bytes }
// }

fn main() {
    unsafe {
        let s = String::from_utf8_unchecked("hello".into());
    }
}