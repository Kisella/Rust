use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Node {
    str: String,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    /*
     * case 88 faker
     * 循环引用造成内存泄漏和栈溢出
     */

    let a = Some(Rc::new(RefCell::new(Node {
        str: "hello".to_string(),
        next: None,
    })));
    assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 1);   //  strong_count a: 1 
    let b = Some(Rc::new(RefCell::new(Node {
        str: "world".to_string(),
        next: None,
    }))); //  b指向a
    assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 1);
    {
        let a_ref = Rc::clone(a.as_ref().unwrap());
        let b_ref = Rc::clone(b.as_ref().unwrap());
        assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 2);//  strong_count a: 2 
        assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 2);

        (*a_ref.borrow_mut()).next = Some(b_ref.clone());   //  借助内部可变性使a指向b
        println!("{a:?}"); 
        (*b_ref.borrow_mut()).next = Some(a_ref.clone());   //  借助内部可变性使b指向a
        // println!("{b:?}"); 
        assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 3);//  strong_count a: 3 
        assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 3);
    }
    assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 2);//  strong_count a: 3 
    assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 2);
    println!("{a:?}");   
    println!("{b:?}"); 
    println!("Stack overflow!");
}
