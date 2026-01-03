fn main() {
    fn swap<T, U>(a: T, b: U) -> (U, T) {
        (b, a)
    }

    // 调用
    swap::<i32, i32>(1, 2);
    swap::<u8, i32>(1, 2);
    swap::<f64, f64>(3.14, 3.14);

    println!("{:p}", swap::<i32, i32> as fn (i32, i32) -> (i32, i32));
    println!("{:p}", swap::<u8, i32> as fn (u8, i32) -> (i32, u8));
    println!("{:p}", swap::<f64, f64> as fn (f64, f64) -> (f64, f64));
    // 三个函数指针的地址必定不同
}