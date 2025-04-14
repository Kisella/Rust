/*
 * 函数传参时发生的所有权移动
 */

fn main() {
    
    /*
     * case 5
     * stack上的值传递给函数时，发生值的复制，动作发生后原变量依然有效
     */
    println!("case 4:");
    let num1 = 6;
    let num2 = add_one(num1);
    println!("num1: {num1}");
    println!("num2: {num2}");

    /*
     * case 6 反例
     * heap上的值传递给函数时，发生所有权移动，动作发生后原变量失效
     */
    println!("\ncase 5:");
    let s1 = String::from("Hello");
    let s2 = append_world(s1);
    // println!("s1: {s1}");    //  使用了失效后的变量
    println!("Compiling error");
    println!("s2: {s2}");

    /*
     * case 7
     * 使用clone方法，深拷贝一份新的值传递给函数，可避免所有权移动，动作发生后原变量依然有效
     */
    println!("\ncase 5:");
    let s1 = String::from("Hello");
    let s2 = append_world(s1.clone());  //  diff from case 6
    println!("s1: {s1}");
    println!("s2: {s2}");
}

fn add_one(mut num: i32) -> i32 {   
    num = num + 1;
    num
}

fn append_world(mut s: String) -> String {
    s.push_str(" World");
    s
}
