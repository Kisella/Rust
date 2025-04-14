/*
 * 声明了不可变引用后, 值的可变引用将失效
 * 推论：值的不可变引用可以存在多个，并且可以同时用于读取值
 * 失效后的引用不能再继续使用
 */ 
fn main() {
    /*
     * case 14 
     * 声明了不可变引用后, 值的可变引用将失效
     * 失效后的引用不能再继续使用
     */ 
    let mut num = 8;
    let ref_mut = &mut num;
    println!("{ref_mut}");
    let ref1 = & num;   // num_ref1 失效
    // println!("{ref1}");    //  使用失效后的引用
    println!("Compiling error!");
    let ref2 = & num;   
    let ref3 = & num;  
    println!("{ref1} {ref2} {ref3}");   //  声明新的不可变引用将不影响原有的不可变引用

    /*
     * case 15 
     * 值的读取动作发生后, 值的可变引用将失效
     * 失效后的引用不能再继续使用
     * 实际上，值的读取动作通常都是借助不可变引用完成的
     */ 
    let mut num = 8;
    let ref_mut = &mut num;
    println!("{ref_mut}");
    println!("{num}");      //  读取值
    // println!("{ref_mut}");   //  使用失效后的引用
    println!("Compiling error!");
}
