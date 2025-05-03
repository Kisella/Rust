use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use super::tree_node::*;

impl<T: Clone + Display> BinaryTree<T> {
    pub fn level_order_traversal(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut curr = self.root.clone();
        let mut queue = VecDeque::new();
        let mut res = Vec::new();

        queue.push_back(curr);
        while !queue.is_empty() {
            curr = queue.pop_front().unwrap();
            res.push(TreeNode::value(&curr));
            let lchild = TreeNode::left(&curr);
            let rchild = TreeNode::right(&curr);
            if lchild.is_some() {
                queue.push_back(lchild);
            }
            if rchild.is_some() {
                queue.push_back(rchild);
            }
        }
        res
    }

    pub fn pre_order_traversal(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut curr = self.root.clone();
        let mut stack = vec![curr];
        let mut res = Vec::new();

        while !stack.is_empty() {
            curr = stack.pop().unwrap();
            res.push(TreeNode::value(&curr));
            if !TreeNode::is_right_empty(&curr) {
                stack.push(TreeNode::right(&curr));
            }
            if !TreeNode::is_left_empty(&curr) {
                stack.push(TreeNode::left(&curr));
            }
        }
        res
    }

    pub fn mid_order_traversal(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut curr = self.root.clone();
        let mut stack = Vec::new();
        let mut res = Vec::new();

        while !stack.is_empty() || curr.is_some() {
            if curr.is_some() {
                stack.push(curr.clone());
                curr = TreeNode::left(&curr);
            } else {
                curr = stack.pop().unwrap();
                res.push(TreeNode::value(&curr));
                curr = TreeNode::right(&curr);
            }
        }
        res
    }

    pub fn post_order_traversal(&self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }
        let mut prev: Option<Rc<RefCell<TreeNode<T>>>> = self.root.clone();
        let mut stack = Vec::new();
        let mut res = Vec::new();
        stack.push(prev.clone());

        while !stack.is_empty() {
            let curr = stack.last().unwrap().clone();
            if !TreeNode::is_left_empty(&curr)
                && !Rc::ptr_eq(prev.as_ref().unwrap(), &TreeNode::left(&curr).unwrap())
                && (TreeNode::is_right_empty(&curr) || !Rc::ptr_eq(prev.as_ref().unwrap(), &TreeNode::right(&curr).unwrap()))
            {
                stack.push(TreeNode::left(&curr));
            } else if !TreeNode::is_right_empty(&curr)
                && !Rc::ptr_eq(prev.as_ref().unwrap(), &TreeNode::right(&curr).unwrap())
            {
                stack.push(TreeNode::right(&curr));
            } else {
                res.push(TreeNode::value(&curr));
                prev = stack.pop().unwrap();
            }
        }
        res
    }

    pub fn pre_order_creat(mut source: VecDeque<Option<T>>) -> BinaryTree<T> {
        let mut tree = BinaryTree::new();
        if source.is_empty() {
            return tree;
        }
        tree.root = TreeNode::new(source.pop_front().unwrap().unwrap());

        let mut stack = Vec::new();
        let mut curr = tree.root.clone();
        let mut flag = false;    //   用于标识curr节点的左子树是否已访问


        while !source.is_empty() {
            let value = source.pop_front().unwrap();
            if !flag && value.is_none() {
                flag = true;
            } else if !flag && value.is_some() {
                TreeNode::left_change(&curr, value);
                stack.push(curr.clone());
                curr = TreeNode::left(&curr);
            } else if flag && value.is_some() {
                 TreeNode::right_change(&curr, value);
                 curr = TreeNode::right(&curr);
                 flag = false;
            } else if !stack.is_empty() {
                curr = stack.pop().unwrap();
            }
        }
        tree
    }
}
