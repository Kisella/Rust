use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use super::tree_node::*;

impl<T: Clone + Display> BinaryTree<T> {
    pub fn level_order_traversal(&self) {
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

    pub fn pre_order_traversal(&self) {
        if self.root.is_none() {
            return;
        }
        // getfirst
        let mut curr = self.root.clone();
        let mut stack = vec![];
        if !TreeNode::is_right_empty(&curr) {
            stack.push(TreeNode::right(&curr));
        }
        if !TreeNode::is_left_empty(&curr) {
            stack.push(TreeNode::left(&curr));
        }

        let mut get_next = |curr: &mut Option<Rc<RefCell<TreeNode<T>>>>| {
            *curr = stack.pop().unwrap();
            if curr.is_none() {
                return;
            }
            if !TreeNode::is_right_empty(&curr) {
                stack.push(TreeNode::right(&curr));
            }
            if !TreeNode::is_left_empty(&curr) {
                stack.push(TreeNode::left(&curr));
            }
        };
        while curr.is_some() {
            println!("{}", TreeNode::value(&curr));
            get_next(&mut curr);
        }
    }

    pub fn mid_order_traversal(&self) {
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
            } else {
                *curr = stack.pop().unwrap()
            }
        };
        while curr.is_some() {
            println!("{}", TreeNode::value(&curr));
            get_next(&mut curr);
        }
    }

    pub fn post_order_traversal(&self) {
        if self.root.is_none() {
            return;
        }
        // getfirst
        let mut curr = self.root.clone();
        let mut stack = vec![];
        loop {
            while !TreeNode::is_left_empty(&curr) {
                stack.push(curr.clone());
                curr = TreeNode::left(&curr)
            }
            if !TreeNode::is_right_empty(&curr) {
                stack.push(curr.clone());
                curr = TreeNode::right(&curr)
            } else {
                break;
            }
        }

        let mut get_next = |curr: &mut Option<Rc<RefCell<TreeNode<T>>>>| {
            *curr = stack.pop().unwrap();
            if curr.is_none() {
                return;
            }
            let mut next = curr.clone();
            loop {
                if !TreeNode::is_right_empty(&next) {
                    next = TreeNode::right(&next);
                    stack.push(next.clone());
                } else {
                    break;
                }
                while !TreeNode::is_left_empty(&next) {
                    stack.push(next.clone());
                    next = TreeNode::left(&next)
                }
            }
        };
        while curr.is_some() {
            println!("{}", TreeNode::value(&curr));
            get_next(&mut curr);
        }
    }
}
