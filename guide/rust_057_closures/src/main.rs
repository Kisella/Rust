use std::thread;

/*
 * 闭包可以以三种方式捕获引用或移动所有权：
 * 1、不可变引用 &
 * 2、可变引用 &mut
 * 3、取得所有权
 */
fn main() {
    // 通过不可变引用的方式捕获环境中的值
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); //  本例中相当与通过不可变引用的方式捕获了环境中的list
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // 通过可变引用的方式捕获环境中的值
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7); //  闭包在定义时捕获了list的可变引用
    // println!("Before calling closure: {list:?}");    // Compiling error, 从闭包的定义开始到最后一次调用结束，都不能存在其他引用，否则将违反引用规则
    borrows_mutably();
    // println!("Between calling closure: {list:?}");    // Compiling error
    borrows_mutably();
    println!("After calling closure: {list:?}"); //  闭包最后一次调用结束后，可变引用就结束了，之后就可以存在list的其他引用了

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // 闭包内部不捕获外部变量，而是通过参数传递
    let mut borrows_mutably = |list: &mut Vec<i32>| list.push(7); 
    println!("Before calling closure: {list:?}");    
    borrows_mutably(&mut list);
    println!("Between calling closure: {list:?}");   

    // 获得数据的所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // 在闭包的参数列表前加上move关键字，这样在定义闭包的时候就会使用移动所有权的方式来捕获环境中的变量
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After defining clousure: {list:?}");  //  Compiling error, list的所有权已被闭包捕获

    // thread新线程有可能在主线程结束之后才结束，若主线程先结束而list的所有权还保留在主线程里的话，list就会被被释放导致thread新线程里的引用失效。故为了线程安全，定义闭包时需要使用捕获所有权的形式
}
