fn show_str(s: &str) {
    println!("{s}");
}

fn main() {
    let s1 = "Hello";
    let s2 = String::from("World");

    show_str(s1);
    // show_str(s2);        //  ❌ 类型不匹配
    show_str(s2.as_ref());  // ✅ 使用 as_ref() 将 String 转换为 &str
    show_str(&s2);          // ✅ 使用 & 操作符将 String 转换为 &str
}