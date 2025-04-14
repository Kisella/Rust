/*
 * 防止循环引用，可把Rc<T>换成Weak<T>
 * Rc<T>实例调用Rc::downgrade函数可以创建值的Weak Reference, 返回类型是Weak<T>, 然后为weak_count计数加1
 * Rc<T>使用weak_count来追踪存在多少Weak<T>
 * 当值离开作用域时，weak_count不为零不影响Rc<T>实例的清理
 * 当Strong Refernen数量为0的时候，Weak Reference会自动断开，故使用Weak<T>前需保证它指向的值仍然存在, 在Weak<T>实例上调用upgrade方法，返回的是Option<Rc<T>>, 进而可完成这个验证
 */

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,    //  parent已有指向children的强引用，为避免循环引用，使用Weak<T>

    /*
     * 使用Rc<T>使得所有子结点都能共享所有权
     * 外层使用RefCell<T>是为了能够灵活地改动父子关系
     * 子节点不止一个，故使用Vec
     */  
    children: RefCell<Vec<Rc<Node>>>,   
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),  //  父节点为空的弱引用
        children: RefCell::new(vec![]),     //  叶子结点的叶子为空，故vec为空，
    }); 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),      //  父节点为空的弱引用
        children: RefCell::new(vec![Rc::clone(&leaf)]),     //  叶子结点, 将leaf的强引用写入
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //  调用Rc::downgrade函数创建branch的弱引用，利用内部可变性，将leaf实例的parent字段更改为branch的弱引用

    println!("leaf = {:#?}", leaf);
    println!("branch = {:#?}", branch);
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}
