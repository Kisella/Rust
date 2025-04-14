fn main() {

    let s = String::from("Hello World!");

    take_ownership(s);      //  s的所有权会移动到take_ownership
    // println!("{}", s);    //  Compilation failure

    let x =  5;
    makes_copy(x);
    println!("x: {}", x);

    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  //  所有权移动：main -> takes_and_gives_back -> main
    println!("{}", s3);

    let (s4, len) = calculate_length(s3);
    print!("The length of '{}' is {}", s4, len);

}

//  函数参数的所有权移动
fn take_ownership(some_string:String) {     //  s的所有权移动到take_ownership的some_string
    println!("{}", some_string);
}   //  some_string的作用域结束，被释放

fn makes_copy(some_number:i32) {
    println!("{}", some_number);
}

//  函数返回值的所有权移动
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string     //  main调用了gives_ownership，some_string的所有权会移动到main
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// 一个变量的所有权总是遵循同样的模式：
//     - 把一个值赋给其他变量是会发生移动
//     - 当一个包含heap的数据的变量离开作用域时，它的值将被释放，除非数据的所有权移动已到另一个变量上