/*
 * case 73
 *  闭包可以选择捕获或者传入的方法来获取变量
 *  捕获，在闭包定义就会发生值的引用或移动所有权
 *  传入，在闭包被调用时才会发生值的引用或所有权的移动
 */ 
fn main() {
    println!("case 73");
    let mut s = String::from("Hello");
    //  这里闭包通过捕获的方式取得了s的可变引用
    let mut add_suffix = || s.push_str(" world");
    // println!("{s}");   //   这里不能打印s，否则闭包add_suffix将会失效
    add_suffix();
    println!("{s}\n");

    let mut s = String::from("Hello");
    let s_ref = &s;
    //  这里闭包被定义为传入参数方式的以取得s的可变引用
    let mut add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s_ref}");   //   在调用闭包add_suffix前，s的可变引用还未被声明，可以正常打印
    add_suffix(&mut s);
    // println!("{s_ref}");   //   Compiling error, 在调用闭包add_suffix后，s的可变引用会被调用，其他的所有引用会失效。
}
