use std::{cell::RefCell, rc::{Rc, Weak}};
#[derive(Debug, Clone)]
enum NodePtrType {
    STRONG(Rc<RefCell<Node>>),
    WEAK(Weak<RefCell<Node>>),
}
use NodePtrType::*;
#[derive(Debug, Clone)]
struct Node {
    str: String,
    next: Option<NodePtrType>,
}

fn main() {
    /*
     * case 89
     * 调用Rc::downgrade创建弱引用，从而避免循环引用
     * Weak<T>用于需要反向引用但不拥有所有权的场景
     * Rc<T>管理所有权，确保数据存活，而Weak<T>允许访问但不影响生命周期，两者互补
     */ 
    let a = Some(Rc::new(RefCell::new(Node {
        str: "hello".to_string(),
        next: None,
    })));
    assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 1);   
    let b = Some(Rc::new(RefCell::new(Node {
        str: "world".to_string(),
        next: None,
    }))); //  b指向a
    assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 1);   
    {
        let a_ref = Rc::clone(a.as_ref().unwrap());
        let b_ref = Rc::clone(b.as_ref().unwrap());
        (*a_ref.borrow_mut()).next = Some(STRONG(Rc::clone(&b_ref))); 
        (*b_ref.borrow_mut()).next = Some(WEAK(Rc::downgrade(&a_ref)));
        assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 2); 
        assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 3);
        assert_eq!(Rc::weak_count(a.as_ref().unwrap()), 1);
        assert_eq!(Rc::weak_count(b.as_ref().unwrap()), 0);
        
    }
    assert_eq!(Rc::strong_count(a.as_ref().unwrap()), 1); 
    assert_eq!(Rc::strong_count(b.as_ref().unwrap()), 2);
    assert_eq!(Rc::weak_count(a.as_ref().unwrap()), 1);
    assert_eq!(Rc::weak_count(b.as_ref().unwrap()), 0);
    println!("{a:#?}");
    println!("{b:#?}");

    if let WEAK(ptr) = (*b.as_ref().unwrap().borrow()).next.as_ref().unwrap() {
        assert_eq!(&(*ptr.upgrade().unwrap().borrow()).str, "hello");   //  b存有a的弱引用，可以因此来读取a中的数据
    }
}
