fn main() {
    let s1 = String::from("hello");
    let s2 = s1;    //  发生移动，s1被标记为movd
    // println!("{s1}, {s2}");  // 拒绝，s1 不能再使用
}