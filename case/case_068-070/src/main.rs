use std::thread;

fn main() {
    /*
     * case 68
     * 闭包可通过不可变借用的方式捕获变量
     * 当闭包的参数省略时，闭包会默认捕获所需变量的引用，这种闭包实现了Fn或FnMut特性，可以被调用多次
     * 被闭包捕获的引用失效时，闭包不能再被调用
     */
    println!("case 68");
    let mut list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    //   闭包变量only_borrows在定义时就捕获了list的不可变引用
    let only_borrows = || println!("From clouser: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    only_borrows();
    println!("After calling closure: {list:?}");
    list.push(4);       //  list的值被改变，所有的list的引用都会失效，包括被闭包捕获的引用
    println!("After push list: {list:?}");
    // only_borrows();  //   Compiling error, only_borrows捕获的引用失效，不能被再调用

    // 重新声明闭包以捕获新的不可变引用
    let only_borrows = || println!("From clouser: {list:?}");   
    only_borrows();

    /*
     * case 69
     * 闭包可通过可变借用的方式捕获变量
     * 当闭包变量被声明为可变时，闭包根据需要捕获所需变量的可变引用，这种闭包实现了FnMut特性，可以被调用多次
     * 被闭包捕获的引用失效时，闭包不能再被调用
     */
    println!("\ncase 69");
    let mut list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    //  闭包声明时的mut关键字是必须的，然后就可以捕获了list的可变引用了
    let mut borrows_mutably = |num| list.push(num);  
    borrows_mutably(4);
    borrows_mutably(5);

    println!("After calling closure: {list:?}");  //  打印时list的不可变引用被调用，原先被定义的所有的list的可变引用都会失效，包括被闭包捕获的引用
    // borrows_mutably(5);  //   Compiling error, borrows_mutably捕获的引用失效，不能被再调用

    // 重新声明闭包以捕获新的可变引用
    let mut borrows_mutably = |num| list.push(num);  
    borrows_mutably(6);
    println!("After calling closure: {list:?}");

    /*
     * case 70
     * 闭包可通过获取所有权的方式捕获变量
     * 使用move关键字，闭包会捕获所需变量的所有权，这种闭包实现了FnOnce特性，只可以被调用一次
     */
    println!("\ncase 70");
    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    // 将闭包传递给了新建立的线程
    // 这里的闭包必须使用move关键字将所有权转移出去，不能只传递不可变引用。因为主线程可能会比新线程现结束，若不转移所有权就会导致悬垂引用
    // 所以编译器会要求将所有权移动到新线程以保证引用的有效性
    thread::spawn(move || println!("From thead: {list:?}"))
        .join()
        .unwrap();
}
