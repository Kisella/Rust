use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use super::tree_node::*;

impl<T: Clone + Display> BinaryTree<T> {
    pub fn level_order_tree(&self) {
        if self.is_empty() {
            return;
        }
        let mut curr = self.root.clone();
        let mut queue = VecDeque::new();
        queue.push_back(curr);
        while !queue.is_empty() {
            curr = queue.pop_front().unwrap();
            println!("{}", TreeNode::value(&curr));
            let lchild = TreeNode::left(&curr);
            let rchild = TreeNode::right(&curr);
            if lchild.is_some() {
                queue.push_back(lchild);
            }
            if rchild.is_some() {
                queue.push_back(rchild);
            }
        }
    }

    pub fn mid_order_tree(&self) {
        if self.root.is_none() {
            return;
        }
        // getfirst
        let mut curr = self.root.clone();
        let mut stack = vec![];
        while !TreeNode::is_left_empty(&curr) {
            stack.push(curr.clone());
            curr = TreeNode::left(&curr)
        }
        
        let mut get_next = |curr: &mut Option<Rc<RefCell<TreeNode<T>>>>| {
            if !TreeNode::is_right_empty(curr) {
                let mut next = TreeNode::right(curr);
                while !TreeNode::is_left_empty(&next) {
                    stack.push(next.clone());
                    next = TreeNode::left(&next);
                }
                *curr = next
            } else if !stack.is_empty() {
                *curr = stack.pop().unwrap()
            } else {
                *curr = None
            }
        };
        while curr.is_some() {
            println!("{}", TreeNode::value(&curr));
            get_next(&mut curr);
        }
    }
}
