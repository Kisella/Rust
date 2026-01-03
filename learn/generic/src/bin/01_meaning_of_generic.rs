fn swap_i32(a: i32, b:i32) -> (i32, i32) {
    (b, a)
}

fn swap_f64(a: f64, b:f64) -> (f64, f64) {
    (b, a)
}

fn swap_string(a: String, b:String) -> (String, String) {
    (b, a)
}

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    
}