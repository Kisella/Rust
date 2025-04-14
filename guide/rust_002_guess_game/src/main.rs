use std::io;
fn main() {
    println!("猜数游戏！");
    println!("猜测一个数:");
    let mut guess;      //  声明一个可变的变量guess
    guess = String::new();      //  创建一个空的字符串实例，并将其赋值给guess
    io::stdin().read_line(&mut guess)   //     调用io库里的stdin函数的read_line方法,读取一行东西, 放到变量guess里
    .expect("无法读取行");  //  调用stdin函数的expect方法，当读取发生错误时，输出提示信息
    println!("你猜测的数是：{}", guess);

    // let foo = 1;
    // foo = 2;    //  所有变量在默认情况下是不可变的，如需修改需在声明时添加mut关键字

    // let mut guess = String::new();
    // 字符串类型就是String, 由标准库所提供，内部使用utf-8格式编码，可以按照需求拓展自己的大小
    // 两个冒号(::)表明new是String类型的一个关联函数，关联函数相当于java中的静态方法

    // io::stdin().read_line(&guess).expect("无法读取行");
    // 引用(&)默认情况下也是不可变的，所以通常也需要添加mut关键字

    // io::stdin().read_line(&mut guess).expect("无法读取行");
    // read_line的返回值类型为 io::Result ,它是一个枚举类型，值为 OK 或 Err
    // expect是io::Result类型定义的一个方法，假如io::Result实例返回的值是Err，则中断当前程序，并将传入的字符串信息所显示出来

    // let msg = "hello world!";
    // println!("你猜测的数是：{}{}", guess, msg);
    // {}是一个占位符，在输出的是否会被替换后边变量的值，如果有多个{}，则会被后边的变量一次替换
}
