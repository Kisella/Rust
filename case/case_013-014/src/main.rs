/*
 * 已经确定引用类型的变量不会自动转化类型
 * 可变引用的函数入参，要求严格匹配
 */ 
fn main() {
    /*
     * case 13 
     * 已经确定引用类型的变量不会自动转化类型
     */
    println!("\ncase 13:");
    let mut x = 0;
    let mut y = 1;
    let mut reference = &x;
    println!("{reference}");
    reference = &mut y; //   reference是可变的，故可以变化绑定为y的引用，但需要注意，这里的mut关键字是不无效的, 因为reference的类型已经被确定为不可变的&i32
    println!("{reference}");
    // *reference += 1;   //   reference的类型依然是&i32不可变引用，故不能修改y的值
    println!("Compiling error!");
    let reference = &mut y; //  重新声明变量接收y的可变引用，这样一来就可以借助可变引用来改变y的值了
    *reference += 1;
    println!("{reference}");

    /*
     * case 14 
     * 可变引用的函数入参，要求严格匹配
     */
    let mut n = 1;
    // add_one(&n); //  Compiling error, 参数不匹配
    add_one(&mut n); //   可变变量的可变引用n, 才能作为add_one的参数
    print_num(&mut n);  //  rust认为将可变引用转为为不可变引用是安全的，故接受了不可变引用入参
}

fn add_one(n: &mut i32) {      //   入参只接受可变引用
    *n += 1;
}
fn print_num(n: &i32) {    //   入参只接受不可变引用和可变引用
    println!("{n}");
}
