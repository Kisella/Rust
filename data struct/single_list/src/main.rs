use single_list::model::*;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let node2 = Node {
        value: 555,
        next: None,
    };
    let node1 = Node {
        value: 666,
        next: Some(Rc::new(RefCell::new(node2))),
    };
    let node0 = Node {
        value: 777,
        next: Some(Rc::new(RefCell::new(node1))),
    };
    println!("{node0:#?}");

    let mut list = List{head: Some(Rc::new(RefCell::new(node0)))};

    let a = Node::next(list.head.as_ref().unwrap());
    let b = Node::next(a.as_ref().unwrap());
    let c = Node::next(b.as_ref().unwrap());
    println!("{:?}", list.head);
    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");
    // let d = Node::next(&c);
    println!("last: {:?}", Node::last(list.head.as_ref().unwrap()));
    

    // node0
    // node0.next.clone()
    // node0.next.clone().unwrap()
    // *node0.next.clone().unwrap().borrow_mut
    list.append(1109);
    println!("list: {:#?}", list.head);

    println!("list[2]: {:#?}", list.index(2));

    list.delete(0);
    println!("list: {:#?}", list.head);

    list.push(100);
    println!("list: {:#?}", list.head);
    println!("{}", list.pop().unwrap());

    list.search(555).unwrap();


}
