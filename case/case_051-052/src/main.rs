/*
 * case 51
 * 生命周期的本质是描述返回值的来源路径
 * 对于返回值引用明确不来源于的那些入参引用，不用标识生命周期
 */
fn boo<'a>(x: &'a str, y: &str) -> &'a str {
    // 返回值的引用明确不来源与y， y不标识生命周期
    println!("print y: {y}");
    x
}

/*
 * case 52 truth
 * 有多少个引用作为返回值，就要声明多少个生命周期参数
 */
fn retTworef<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
    (x, y)
}

/*
 * case 52 faker1
 * 不要声明多于返回值引用个数的生命周期参数, 否则可能造成生命周期不匹配
 */
// fn longest2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }


/*
 * case 52 faker2
 * 不同的返回值使用相同的生命周期会造成不必要的限制
 */
fn retTworef1<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
    (x, y)
}
fn main() {
    println!("case 51");
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    let result = boo(&str1, &str2);
    println!("{result}");
    drop(str2); //      str2的失效不影响result   
    println!("{result}");
    drop(str1); //      str1的失效, result 也失效
    // println!("{result}");     //  Compiling error, 使用了失效后的引用

    println!("\ncase 52 truth");
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    let (x, y) = retTworef(&str1, &str2);
    println!("{x} {y}");
    drop(str1);
    // println!("{x}");    //  Compiling error, 使用了失效后的引用，x明确来源于str1
    println!("{y}");    //  y明确来源于str2, 故str1的失效不会影响到y

    println!("\ncase 52 faker");
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    let (x, y) = retTworef1(&str1, &str2);
    println!("{x} {y}");
    drop(str1);
    // println!("{y}");    y的来源于str2，本不应该受到str1失效的影响，但由于生命周期参数的错误使用，使编译器认为y也可能失效，故拒绝了y的使用。这就造成了不必要的限制。
}
