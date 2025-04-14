use std::fmt::Display;

fn main() {
    /*
     * case 57
     * 所有的字符串字面值都具有'static 生命周期
     * 'static 称为静态生命周期，它表示受影响的引用可以在整个程序的持续时间内存活
     */
    println!("case 57");
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    println!("\ncase 58");
    let str1 = String::from("Hello wrold!");
    let str2= String::from("Maybe a longer string");
    longest_with_an_announcement(&str1, &str2, 5201314);
}

/*
 * case 58
 * 生命周期参数、泛型类型参数一起在尖括号里声明
 * 生命周期参数必须声明在泛型参数前面
 */
fn longest_with_an_announcement<'a, T: Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

