use std::{thread, time::Duration};

/*
 *                         闭包的类型推断和注释
 *   闭包，又称为匿名函数，与fn函数类似有参数和返回值，但也有所不同：
 * 1、闭包不会暴露给用户的接口中使用
 * 2、闭包通常不需要标注参数或返回值类型（需要时也可以添加类型注释）
 * 3、闭包通常很短，只在有限的上下文中使用，以便编译器可以推断其参数和返回值的类型
 */
fn main() {
    //  定义一个闭包，这个闭包显式指明了参数和返回值，定义一个闭包时函数体并不会执行
    let expensive_clousure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };      //  定义闭包是一个语句，右大括号后要有分号
    expensive_clousure(10); //  调用闭包

    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    // let n = example_closure(5);    // Compiling error, 20行调用闭包时编译器已经推断出输入类型为String, 闭包的参数确定后将不再改变。此行在调用时传入整形参数，与期望的String类型不匹配

    // Rust允许在闭包参数中进行模式匹配， 包括使用下划线
    let f = |_| ();   //   厕所闭包
    let s = String::from("Hello");
    f(s);
    // println!("{}", s);  //  Compiling error, f 导致 s 立即被丢弃
}

// fn add_one_v1    (x: u32) -> u32 { x + 1 }      //  函数
// let add_one_v2 = |x: u32| -> u32 { x + 1 };     //  闭包，显式指明参数和返回类型
// let add_one_v3 = |x|             { x + 1 };     //  闭包，隐式指明参数和返回类型
// let add_one_v4 = |x|               x + 1  ;     //  闭包，由于函数体只有一条语句，可进一步省略大括号
// add_one_v3和add_one_v4, 这两个闭包没有指明参数和返回值类型, 要求编译器能够在使用时推断出其对应的参数类型和返回类型。且一旦确定后，就不能再改变。