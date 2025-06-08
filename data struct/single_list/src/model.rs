use std::{cell::*, rc::*};

pub struct List {
    pub head: Option<Rc<RefCell<Node>>>,
}
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Option<Rc<RefCell<Node>>> {
        Some(Rc::new(RefCell::new(Node { value, next: None })))
    }

    pub fn get(curr: &Rc<RefCell<Node>>) -> i32 {
        (*curr.borrow()).value
    }

    #[inline(always)]
    pub fn next(curr: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        (*curr.borrow()).next.clone()
    }

    pub fn last(head: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut curr = head.clone();

        while (*curr.borrow()).next.is_some() {
            curr = Node::next(&curr).unwrap();
        }
        curr
    }
}

impl List {
    pub fn append(&mut self, value: i32) {
        let new_node = Node::new(value);
        if self.head.is_none() {
            return (*self).head = new_node;
        }
        let last = Node::last(&self.head.as_ref().unwrap());
        (*last.borrow_mut()).next = new_node;
    }

    pub fn index(&self, index: usize) -> Result<Option<Rc<RefCell<Node>>>, &'static str> {
        let mut curr = self.head.clone();
        let mut i = 0;
        while curr.is_some() && i < index {
            curr = Node::next(curr.as_ref().unwrap());
            i += 1;
        }
        match i == index {
            true => Ok(curr),
            false => Err("List::index call error as index out of bounds!"),
        }
    }

    pub fn search(&self, value: i32) -> Result<Option<Rc<RefCell<Node>>>, &'static str> {
        let mut curr = self.head.clone();
        while curr.is_some() {
            if Node::get(curr.as_ref().unwrap()) == value {
                return Ok(curr);
            }
            curr = Node::next(curr.as_ref().unwrap());
        }
        Err("The value is not found in the list")
    }

    pub fn delete(&mut self, index: usize) {
        if index == 0 && self.head.is_some() {
            return (*self).head = Node::next(self.head.as_ref().unwrap());
        }
        let prev = self.index(index - 1).unwrap();
        let curr = Node::next(prev.as_ref().unwrap());
        let next = Node::next(curr.as_ref().unwrap());
        (*prev.unwrap().borrow_mut()).next = next;
    }

    //  头插(入栈)
    pub fn push(&mut self, value: i32) {
        (*self).head = Some(Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
        })));
    }

    //  头删(出栈)
    pub fn pop(&mut self) -> Result<i32, &'static str> {
        if self.head.is_none() {
            return Err("List::pop call on empty list!");
        }
        let pop = (*self.head.as_ref().unwrap().borrow()).value;
        (*self).head = Node::next(self.head.as_ref().unwrap());
        Ok(pop)
    }

    // pub fn pop(&mut self) -> Option<i32> {
    //     self.head.take().map(|old_head| {
    //         self.head = old_head.borrow().next.clone();
    //         old_head.borrow().value
    //     })
    // }
}
