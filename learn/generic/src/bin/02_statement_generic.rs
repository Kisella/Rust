fn swap_tuple<T, U>(tp: (T, U)) -> (U, T) {
    let t: T = tp.0;
    let u: U = tp.1;
    (u, t)
}

fn main() {
    let tp_1: (i32, String) = (15, String::from("hello"));
    let tp_2: (String, i32) = swap_tuple(tp_1);
}