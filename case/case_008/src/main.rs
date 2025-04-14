/*
 * 重名变量作用域屏蔽
 * 编程实践：不同作用域下的命名的变量不要重名以避免混淆，小范围的变量不要去屏蔽大范围的变量。
 */
fn main() {
    /*
     * case 8 faker 
     * 重名变量作用域屏蔽
     */ 
    println!("case 7 faker:");
    let s1 = String::from("hello");
    let s2;
    {
        let s1 = String::from("world");  //  内部定义了一个重名变量s1，外部的s1变量将被屏蔽
        s2 = s1;
        // println!("s1: {s1}");   // 内部变量s1所有权已移动给s2
        println!("Compiling error!");
    }
    // 大括号结束，内部的变量s1作用域结束
    println!("s1: {s1}");   // 大括号结束，外部的s1变量恢复使用 
    println!("s2: {s2}");

    /*
     * case 8 truth
     * 重名变量作用域屏蔽
     */ 
    println!("\ncase 7 truth:");
    let s1 = String::from("hello");
    let s2;
    {
        let s3 = String::from("world");  //  内部定义了一个重名变量s1，外部的s1变量将被屏蔽
        s2 = s3;
        println!("s1: {s1}");
        // println!("Compiling error!");
    }
    // 大括号结束，内部的变量s1作用域结束
    println!("s1: {s1}");  
    println!("s2: {s2}");
}
