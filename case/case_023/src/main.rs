/*
 * 声明在函数中的变量返回其引用, 在Rust中是被绝对禁止的行为
 * 声明在函数中的值可以返回其所有权
 */ 
fn main() {
    println!("Hello, world!");
}

/*
 * case 23 faker
 * 
 */
fn return_a_string1() -> &String {
    let s = String::from("Hello world");
    let s_ref = &s;
    s_ref   //  return_a_string调用结束后s将被清理，返回的s_ref是必然失效的
}

/*
 * case 23 truth
 * 声明在函数中的值可以返回其所有权
 */
fn return_a_string2() -> String {
    let s = String::from("Hello world");
    s
}