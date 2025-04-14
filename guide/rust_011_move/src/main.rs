fn main() {
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("{}", s1);
    // String类型需要在heap中申请内存，当使用一个新的变量来绑定String类型变量时就会发生所有权的移动(Move), 原本的String类型将失效
    // 一块申请的heap内存的所有权只能被一个变量所拥有，从而可以避免一块内存的重复释放

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}",s1);
    // 可以通过clone方法来进行数据的深拷贝，这样做会申请一块新的内存，就不会发生所有权的移动，变量s1依然有效

    let x = 5;
    let y = x;
    let z = x.clone();
    println!("{},{},{}", x, y, z);
    // 对于在stack上的数据则没有所有权的移动的概念
    // 在stack上的变量在编译时就确定了自己的大小，并且将自己的数据完整的存储在stack中，而对于这些数组的复制(copy)操作都是非常快速的。这就意味着在创建了变量y之后没有任何理由去阻止变量x继续保持有效
    // 对于在stack上的数据来说，深拷贝和浅拷贝是没有任何区别的，调用clone方法并不会与直接的浅拷贝有任何行为上的区别

    // 一些拥有Copy trait的类型
    // -任何简单标量的组合类型都是可以Copy的
    // -任何需要分配内存或某种资源的都不是Copy的
    // -一些拥有Copy trait的类型：
    //     -所有的整数类型、浮点类型
    //     -bool
    //     -char
    //     -数组
    //     -Tuple, 且所有字段都是Copy的
    //         -(i32, u32) 是
    //         -(i32, String) 不是
}
