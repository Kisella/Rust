const MAX_POINTS: u32 = 100_000;
// 声明常量使用关键字const关键字，且必须显式标注数据类型
// 常量不可以使用mut, 常量永远都是不可变的
// 常量只可绑定到常量表达式，无法绑定到函数的调用结果或只在运行时才能计算出的值

fn main() {
    println!("常量 MAX_POINTS is {}", MAX_POINTS);

    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    println!("The value of x is {}", x);
    let x = 2 * x;
    println!("The value of x is {}", x);
    // 可以使用相同的名字成名新的变量，新的变量就会隐藏(shadow)之前成名的同名变量
    // 在后续的代码中这个变量名代表的就是新的变量

    let spaces = "     ";
    let spaces = spaces.len();
    // 使用let声明的新变量，它的类型可以与之前不同
    println!("{}", spaces);
}
