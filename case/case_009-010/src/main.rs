/*
 * 传递引用作为参数避免所有权的移动
 */ 
fn main() {
    /*
     * case 9:
     * 函数得到所有权后又交还回来
     */ 
    println!("\ncase 8 :");
    let s1 = String::from("Hello");
    let s1 = greet_and_return(s1);  //  借助遮蔽特性接收返还的所有权，当然，也可选择新建一个不同名的变量来接收
    println!("{s1}");

    /*
     * case 10:
     * 引用作为函数入参，可避免所有权的移动
     * 接收所有权又返还所有权的做法太过繁琐，可考虑借用的方式
     */ 
    println!("\ncase 9 :");
    let s1 = String::from("Hello");
    greet_by_ref(&s1);                  //  将引用传入
    println!("{s1}");
}

fn greet_and_return(s: String) -> String {
    println!("{s}");
    s
}

fn greet_by_ref(s: &String) {
    println!("{s}");
}
