/*
 * 使用Rc<T>和RefCell<T>可能创造出循环引用，进而发生内存泄漏
 * 每个项的引用数量不会变成0, 值就不会被清理掉
 */

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
use crate::List::{Cons, Nil};
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    println!("");

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) =a.tail() {
        *link.borrow_mut() = Rc::clone(&b);     //  使用内部可变性，将a的第二个元素(RefCell<Rc<List>>)指向b。这样一来，就构造出了a指向b, b又指向a的循环引用
    }

    println!("");

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
    // println!("{:?}", a);   //  Process failure
}
