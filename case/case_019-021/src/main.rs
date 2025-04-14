fn main() {
    /*
    * case 19
    * 当值的所有权发生移动时，它的所有引用立即失效
    */
    let s1 = String::from("Hello world");
    let s_ref = &s1;
    println!("{s_ref}");
    let s2 = s1;
    // println!("{s_ref}");  //  Compiling error, 使用了失效后的引用

    /*
    * case 20
    * 当值的作用域结束时，它的所有引用立即失效
    */
    let ref1;
    {
        let num = 10;
        ref1 = &num;
        println!("{ref1}");
    }
    // println!("{ref1}");     //  Compiling error, 使用了失效后的引用
}

 /*
  * case 21
  * 作为形参的引用不会受到声明在函数中的引用的影响
  *
  */
  fn function(v: &mut Vec<i32>) {
    let ref1 = &mut v[1];   //  ref1不会使v失效
    println!("{ref1}");
    let ref2 = &v[0];  //  ref2的声明使ref1失效，但不会使v失效
    // println!("{ref1}");   // 使用失效后的引用
    println!("{v:?}");   // 形参v不受内部声明的引用的影响

    if false {
        let ref3 = &v[3];
        println!("{ref3}");
        v[0] = 1;    //  值发生改变，函数内部定义的所有引用失效
        
        // println!("{ref2}"); // 使用失效后的引用
        // println!("{ref3}");
    } else {
        // 在else分支中值没有通过v发生改变, 引用有效
        let ref3 = &v[3];
        println!("{ref2}");
        println!("{ref3}");
    }
}
