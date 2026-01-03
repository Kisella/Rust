fn main() {
    fn echo<T>(x:T) -> T {
        x
    }

    let a = echo(42);
    let b = echo("hello");

    fn make_number<T>() -> T {
        panic!("something happened!")
    }
    let a: i32 = make_number();
    let b: u8 = make_number();
}