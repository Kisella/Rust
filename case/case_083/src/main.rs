use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}
#[derive(Debug)]
pub struct List(Option<Rc<RefCell<Node>>>);

fn main() {
    /*
     * case 83 
     * Rc嵌套RefCell，实现多个指针对同一块数据的修改
     */ 
    let a = Rc::new(RefCell::new(555));
    let b = Rc::clone(&a);
    let c = Rc::clone(&b);
    println!("a : {a:?}\nb : {b:?}\nc : {c:?}\n");
    *a.borrow_mut() = 666;
    println!("a : {a:?}\nb : {b:?}\nc : {c:?}\n");
    *b.borrow_mut() = 777;
    println!("a : {a:?}\nb : {b:?}\nc : {c:?}\n");
    *c.borrow_mut() = 888;
    println!("a : {a:?}\nb : {b:?}\nc : {c:?}\n");

}
