// Box<T>是最简单的智能指针，允许在heap上存储数据
// 使用Box<T>后在stack分配一个指针，指针指向一块heap内存
use crate::List::{Cons, Nil};
fn main() {
    let a = 5; //  存储在stack上
    let b = Box::new(5); //  存储在heap上
    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print!("{:?}",list);
}

// enum List {
//     Cons(i32, List),
//     Nil,
// }    //  Compiling error, 该递归类型占用无限的空间
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), //  Box<T>是一个指针，指针的大小不会基于它指向的数据的大小变化而变化，Rust就知道List类型需要多少空间
    Nil,
}
