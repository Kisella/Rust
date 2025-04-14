use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    ptr: Box<i32>,
}
fn main() {
    let a = Box::new(666);
    let a = Node {
        value: 555,
        ptr: a,
    };
    println!("{a:#?}");

    let mut a = Rc::new(RefCell::new(a));
    println!("{}",Rc::strong_count(&a));
    let mut a1 = Some(Rc::clone(&a));
    println!("{}",Rc::strong_count(&a));
    let mut a2 = a1.clone();
    println!("{}",Rc::strong_count(&a));
    (*(a2.unwrap()).borrow_mut()).ptr = Box::new(777);
    println!("{a1:?}");
    println!("{a:?}");
    

}
