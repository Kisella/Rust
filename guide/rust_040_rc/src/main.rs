// 有时，一个值可能需要多个所有者(例如图数据结构)，为了支持这一多重所有权的功能，rust提供了Rc<T>类型
// reference couting(引用计数)类型，会在实例内部维护一个用于记录值的引用次数的计数器，从而判断值是否还在使用
// 若追踪到值的引用次数为零，意味着这个值可以被安全地清理掉(不会因此触发引用失效的问题)
// Rc<T>只可用于单线程场景
// Rc<T>通过"不可变引用"，使程序可以在不同部分之间共享只读数据

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};
fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));    //  Compiling error, a的所有权已经为b所持有

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));  //  new函数: 创建一个Rc类型的智能指针
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // 每次调用克隆函数，都会使引用计数加1, a这个智能指针中的数据只有在引用计数减少到零时才会被清理掉

    a.clone();  //  实例的clone方法会进行数据的深拷贝，而关联函数Rc::clone不会进行深拷贝操作只会增加引用计数所以速度较快，惯例上是使用Rc::clone, 而实例的clone方法偶尔使用

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count of a after creating a is {}", Rc::strong_count(&a)); // strong_count函数: 查询引用的强引用计数
    // 当创建a的实例后，a本身就存在对自身的引用，故引用计数为1

    let b = Cons(3, Rc::clone(&a));
    println!("Count of a after creating b is {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count of a after creating b is {}", Rc::strong_count(&a));
    }
    println!("Count of a after c goes out of scope is {}", Rc::strong_count(&a));
}
