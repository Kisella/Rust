use std::io;    //  prelude
use rand::Rng;   //  trait
use std::cmp::Ordering;
fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("神秘数字是:{}", secret_number);

    loop {  //  无限循环
        println!("猜测一个数:");
        let mut guess= String::new();      
        io::stdin().read_line(&mut guess).expect("无法读取行");  

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {                              //      使用通配符下划线(_)表示不关心错误信息
                println!("please type a number!");
                continue;                            //      跳出本次循环，进入下一次循环
            }
        };
        // 使用match来代替expect方法来处理错误是一种惯用手段

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   //  arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  //  使用break关键字跳出loop
            }
        }
    }
}
