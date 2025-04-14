/*
 * 模式存在两种形式: 可失败的、不可失败的，也叫可辩驳的(refutable pattern)、不可辩驳的(irrefutable pattern)
 * 能匹配任何可能传递的值的模式是不可失败的   - 例如: let x = 5;
 * 对某些可能的值，无法进行匹配的模式是可失败的  - 例如: if let Some(x) = a_value;
 * 
 * 函数参数、let语句、for循环只接受不可失败的模式
 * if let 和 while let 接受可失败的和不可失败的模式
 */ 
fn main() {
    let a:Option<i32> = Some(5);
    // let Some(x)  = a;   //  Compiling error, Some(T)是一个可辨驳的模式， 不能与let搭配使用
    if let Some(x) = a {    //  可使用if let与可失败的模式搭配使用
        // do something
    };

    if let x = 5 {};    //  waring, x = 5是不可失败的，总会匹配成功，搭配if let是没有意义的

    let v = 0u8;
    match v {
        1 => println!("one"),   //  可失败的
        3 => println!("three"), //  可失败的
        5 => println!("five"),  //  可失败的
        7 => println!("seven"), //  可失败的
        _ => {}                 //  不可失败的
    }
    // match表达式除了最后一个分支，其他分支应该是可失败的。match表达式最后一个分支可以是可失败的和不可失败的，使用下划线(_)便是不可失败的，若是可失败的，则说明match表达式已经穷举了所有可能
}
