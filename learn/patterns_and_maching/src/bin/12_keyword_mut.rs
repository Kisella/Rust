fn main() {
    let s = String::from("hello");

    match s {
        // 相当与发生了一次 let mut x = s; 字符串重新变为可变，同时发生了所有权的移动
        mut x => x.push_str(" world"),
    }
}