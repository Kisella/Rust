fn main() {
    let x = 0;
    match x {
        42 => println!("It's the meaning of life."),
        _ => println!("Other"),  // 相当于发生一次let _ = x
    }
}