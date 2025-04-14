// Rust中存在类似NULL概念的枚举 - Option<T>
/*
 *  Option<T>在标准库中的定义为:
 *  enum Option<T> {
 *      Some(T),
 *      None,
 *  }
 */
fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
    // 使用None变体时, 编译器无法直接推断出对应类型, 故需要显式声明类型

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;              //    Complying failure
    // 在Rust中Option<T>和T是不同的类型, 不可以把Option<T>当成T的实例
    // 若想使用Option<T>中的T, 则必须将它转为 T
    // Rust 就是使用通过使用Option<T>来避免空值解引用

    let sum = x + y.unwrap_or(0);
    // Option<T>的unwrap_or方法: 如果是Some则返回内部的值, 如果是None则返回提供的默认值
    println!("{}", sum);
}
