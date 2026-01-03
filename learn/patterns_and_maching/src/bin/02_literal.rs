fn main() {
    // 字面量相等即匹配成功
    let x = 42;
    match x {
        42 => println!("It's the meaning of life."),
        0 => println!("Zero"),
        _ => println!("other"),
    }

    // 支持 const / static
    const ZERO: i32 = 0;
    const LIEF: i32 = 42;

    let x = 42;
    match x {
        ZERO => println!("It's zero"),
        LIEF => println!("The meaning of life"),
        _ => println!("Something else"),
    }
}