use NodePtrType::*;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug, Clone)]
pub struct Node {
    pub value: i32,
    pub prev: Weak<RefCell<Node>>,
    pub next: NodePtrType,
}
#[derive(Debug, Clone)]
pub enum NodePtrType {
    WEAK(Weak<RefCell<Node>>),
    STRO(Rc<RefCell<Node>>),
}
#[derive(Debug, Clone)]
pub struct List {
    pub head: Option<Rc<RefCell<Node>>>,
    pub count: usize,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: Weak::new(),
            next: WEAK(Weak::new()),
        }))
    }

    pub fn next(curr: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match &(*curr.borrow()).next {
            WEAK(next) => next.upgrade().unwrap(),
            STRO(next) => next.clone(),
        }
    }

    pub fn prev(curr: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        (&(*curr.borrow())).prev.upgrade().unwrap()
    }
}

impl List {
    pub fn new() -> List {
        List { head: None, count: 0 }
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Node::new(value);
        if self.head.is_none() {
            (*new_node.borrow_mut()).prev = Rc::downgrade(&new_node);
            (*new_node.borrow_mut()).next = WEAK(Rc::downgrade(&new_node));
            (*self).head = Some(new_node);
            self.count += 1;
            return;
        }
        let old_tail = (*self.head.as_ref().unwrap().borrow()).prev.upgrade().unwrap();
        (*new_node.borrow_mut()).next = WEAK(Rc::downgrade(self.head.as_ref().unwrap()));
        (*new_node.borrow_mut()).prev = (*self.head.as_ref().unwrap().borrow()).prev.clone();
        (*self.head.as_ref().unwrap().borrow_mut()).prev = Rc::downgrade(&new_node);
        (*old_tail.borrow_mut()).next = STRO(new_node);
        self.count += 1;
    }

    pub fn node(&self, index: usize) -> Result<Rc<RefCell<Node>>, &'static str> {
        if index >= self.count {
            return Err("List::node call error as index out of bounds");
        }
        let mut curr = self.head.as_ref().unwrap().clone();
        if index <= self.count - index {
            for _ in 0..index {
                curr = Node::next(&curr);
            }
        } else {
            for _ in 0..self.count - index {
                curr = Node::prev(&curr);
            }
        }
        Ok(curr)
    }

    pub fn index(&self, index: usize) -> Result<i32, &'static str> {
        match self.node(index) {
            Ok(curr) => Ok((*curr.borrow()).value),
            Err(_) => Err("List::index call error as index out of bounds")
        }
    }

    pub fn delete(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.count {
            return Err("List::delete call error as index out of bounds");
        }
        if index == 0 {
            let head = self.head.as_ref().unwrap();
            let prev = Node::prev(head);
            let next = Node::next(head);
            (*prev.borrow_mut()).next = WEAK(Rc::downgrade(&next));
            (*next.borrow_mut()).prev = Rc::downgrade(&prev);
            (*self).head = Some(next);
            (*self).count -= 1;
            return Ok(());
        }
        if index == self.count-1 {
            let next = self.head.as_ref().unwrap().clone();
            let prev = Node::prev(&Node::prev(&next));
            (*prev.borrow_mut()).next = WEAK(Rc::downgrade(&next));
            (*next.borrow_mut()).prev = Rc::downgrade(&prev);
            (*self).count -= 1;
            return Ok(());
        }
        let prev = self.node(index-1).unwrap();
        let next = Node::next(&Node::next(&prev));
        (*next.borrow_mut()).prev = Rc::downgrade(&prev);
        (*prev.borrow_mut()).next = STRO(next);
        (*self).count -= 1;
        Ok(())
    }
}
