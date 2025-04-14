use std::{thread, time::Duration};

fn main() {
    /*
     * case 64
     * 闭包是存储在变量中的匿名函数
     * 声明匿名函数的形式为"|参数列表| -> 返回值"，参数列表在两个竖杠(||)之间定义，返回值在->后定义
     * 闭包不需要fn函数那样标注参数和返回值的类型，它的参数和返回值类型编译器可以推断出来
     * 调用闭包时在闭包变量后加上括号(()), 括号里填入参数，与调用fn函数类似
     */
    println!("case 64");
    let x: u32 = 5;
    //  fn函数，需要明确参数和返回值的类型
    //  闭包，可以选择标注参数和返回值的类型
    //  闭包，可以不标注参数和返回值的类型，而由编译器自动推断
    //  闭包，可以省略返回值类型，而由编译器自动推断
    //  闭包，当函数体只有一个语句时可以省略大括号，而fn函数不允许这么做
    //  闭包，甚至可以省略参数，借由“变量捕获”，让函数体能够顺利运行
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    } //   必须标注参数和返回值类型
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; //   选择标注参数和返回值类型
    let add_one_v3 = |x| -> u32 { x + 1 }; //   不标注参数类型
    let add_one_v4 = |x| x + 1; //   省略返回值
    let add_one_v5 = |x| x + 1; //   省略大括号
    let add_one_v6 = || x + 1; //   甚至省略参数(变量捕获)

    println!("{}", add_one_v1(x));
    println!("{}", add_one_v2(x));
    println!("{}", add_one_v3(x));
    println!("{}", add_one_v4(x));
    println!("{}", add_one_v5(x));
    println!("{}", add_one_v6());

    /*
     * case 65
     * 可以为闭包的参数和返回值标注类型
     */
    println!("\ncase 65");
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("{}", expensive_closure(52));

    /*
     * case 66 
     * 编译器会为闭包的参数和返回值推断出唯一的类型
     */
    println!("\ncase 66");
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let s = example_closure(5);   //  Compiling error, 闭包example_closure的入参和返回值已经被推断成String类型。推断一旦完成就不会更改，再传入其他类型的参数会提示参数不匹配

    /*
     * case 67
     * 厕所闭包
     * 在闭包参数中可以进行模式匹配，包括使用下划线
     */
    println!("\ncase 67");
    let str = String::from("Hello World!");
    let num = 56;
    let toilet_string = |_| ();
    let toilet_i32 = |_| ();
    toilet_string(str);
    toilet_i32(num);
    // println!("{str}");   
    // println!("{num}");    //  str和num都已被丢弃
}
