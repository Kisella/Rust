/*
 * 变量赋值时发生的所有权移动
 */
fn main() {
    /*
     * case 2 
     * stack上的值赋值给另一个变量是会发生值的复制，动作发生后原变量依然有效
     */
    println!("case 1: ");
    let num1 = 10;
    let num2 = num1;
    println!("num1: {num1}");
    println!("num2: {num2}");

    /*
     * case 3 反例
     * heap上的值赋值给另一个变量是会发生所有权的移动，动作发生后原变量失效
     */ 
    println!("\ncase 2: ");
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("s1: {s1}");                //  使用了失效后的变量    
    println!("Compiling error");
    println!("s2: {s2}");

    /*
     * case 4
     * 调用clone方法，原变量在heap上的值深拷贝一份给新变量，动作发生后原变量依然有效，没有发生所有权移动
     */ 
    println!("\ncase 3: ");
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {s1}");   
    println!("s2: {s2}");

}
