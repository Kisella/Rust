fn main() {
    let mut s = String::new(); // 创建一个空字符串
    println!("{}", s);

    let data = "inital contents";
    let s = data.to_string();
    let s1 = "inital contents".to_string();
    println!("{s}\n{s1}");

    let s = String::from("inital contents");
    println!("{s}");

    //  字符串拼接
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    let s1 = String::from("bar");
    s.push_str(&s1);
    println!("{s1}"); //  push_str方法不会获取参数的所有权

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    //  加号运算符实现字符串拼接
    let s1 = String::from("Hello, ");
    let s2 = "World!".to_string();
    let s3 = s1 + &s2; //  使用加号实现字符串拼接，加号前的变量会发生所有权移动，
    println!("{s3}");
    // println!("{s1}");        //  Compilying error
    let s4 = s3 + &s2 + &s2;
    println!("{s4}");
    println!("{s2}");

    // format实现字符串拼接
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{} {} {}", s1, s2, s3);   //  format不会获取变量的所有权
}
