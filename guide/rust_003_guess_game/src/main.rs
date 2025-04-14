use std::io;    //  prelude
use rand::Rng;   //  trait
use std::cmp::Ordering;
fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // thread_rng方法的返回类型为ThreadRng，ThreadRng类型是一个随机数生成器，它位于本地线程空间，并通过操作系统获得随机数的种子
    // gen_range是Rng这个接口(trait)里定义的方法，有两个参数，代表最小值和最大值的范围，左闭右开

    println!("神秘数字是:{}", secret_number);
    println!("猜测一个数:");
    let mut guess= String::new();      
    io::stdin().read_line(&mut guess).expect("无法读取行");  

    let guess:u32 = guess.trim().parse().expect("please type a number!");
    // 允许使用同名的新变量来隐藏原来同名的旧变量，从15行以后(16行开始)使用的guess就是这个新定义的guess。这一特性通常是用在类型转换的场景中，这就允许我们复用原本的变量名，而无需去取新的变量名
    // trim方法会去掉两边的空白字符(包括空格和换行符等)
    // 声明变量时后面加冒号跟一个类型，称为显示声明变量类型
    // parse方法可将字符串解析成某种数值类型，由于guess被显示声明为了u32类型，故parse方法会将字符串解析成u32类型

    println!("你猜测的数是：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),   //  arm
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    // 变量上定义有cmp方法，返回类型为Ordering
    // Ordering是一个枚举类型，有Less,Greater和Equal三个值
    // match表达式让我们可以根据cmp返回的Ordering类型的值来决定下一步是做什么
    // match表达式是由多个分支(arm)来组成的，每一个分支都有一个用于匹配的模式(这个例子中就是Ordering类型)，如果match后紧跟着的那个值能跟大0括号里的某个模式匹配上，那么它就会执行相应的arm里的代码
}
