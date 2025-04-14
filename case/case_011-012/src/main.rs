/*
 * 不可变引用与可变引用
 */ 
fn main() {
    /*
     * case 11 
     * 不可变变量仅可声明不可变引用，不可声明可变引用
     * 不可变引用仅可用于读取值
     */ 
    println!("\ncase 11:");
    let x = 0;
    let x_ref = &x;
    // let x_ref_mut = &mut x;      //  Compiling error, 不可变变量不可声明可变引用
    println!("Compiling error!");
    println!("{x} {x_ref}");

    /*
     * case 12 
     * 可变变量可声明不可变引用和可变引用
     * 可变引用可用于读取值和改变值，改变值时需使用解引用符号(*)
     */ 
    println!("\ncase 11:");
    let mut x = 0;
    let x_ref = &x;
    let x_ref_mut = &mut x;
    println!("{x_ref_mut}");
    *x_ref_mut = 10;
    println!("{x_ref_mut}");
    println!("{x}");
}
