fn main() {
    let s: String = String::from("Hello, world!");
    let r1: &String = &s;
    let ref r2: &String = &s;

    println!("原始字符串：{s}");
    println!("引用 r1: {r1}");
    println!("引用 r2: {r2}");

    let mut v = 42;
    let r = &v;
    // *r += 1; // 不能通过共享引用修改值
}