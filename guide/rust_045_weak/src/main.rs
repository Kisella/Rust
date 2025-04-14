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
    println!("After creating leaf, leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),      //  父节点为空的弱引用
            children: RefCell::new(vec![Rc::clone(&leaf)]),     //  叶子结点, 将leaf的强引用写入
        });
        println!("After creating branch, branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("After creating branch, leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //  调用Rc::downgrade函数创建branch的弱引用，利用内部可变性，将leaf实例的parent字段更改为branch的弱引用

        println!("After leaf weak point to branch, branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("After leaf weak point to branch, leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
        // println!("leaf = {:#?}", leaf);
        // println!("branch = {:#?}", branch);
        // println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    }   //  branch离开作用域后，branch的强引用成功降为零，值被清理

    println!("After branch leaves the scope, leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!("After branch leaves the scope, leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
