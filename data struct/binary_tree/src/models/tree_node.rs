use std::{cell::RefCell, rc::Rc};

pub struct TreeNode<T> {
    pub data: T,
    pub lchild: Option<Rc<RefCell<TreeNode<T>>>>,
    pub rchild: Option<Rc<RefCell<TreeNode<T>>>>,
}

pub struct BinaryTree<T> {
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            _ => false,
        }
    }

    pub fn clear(&mut self) {
        (*self).root = None
    }
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(data: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            data,
            lchild: None,
            rchild: None,
        })))
    }

    #[inline]
    pub fn is_left_empty(curr: &Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        match (*curr.as_ref().unwrap().borrow()).lchild {
            None => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_right_empty(curr: &Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        match (*curr.as_ref().unwrap().borrow()).rchild {
            None => true,
            _ => false,
        }
    }

    #[inline]
    pub fn left(curr: &Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        (*curr.as_ref().unwrap().borrow()).lchild.clone()
    }

    #[inline]
    pub fn right(curr: &Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        (*curr.as_ref().unwrap().borrow()).rchild.clone()
    }

    #[inline]
    pub fn left_change(curr: &Option<Rc<RefCell<TreeNode<T>>>>, value: Option<T>) {
        match value {
            Some(value) => (*curr.as_ref().unwrap().borrow_mut()).lchild = TreeNode::new(value),
            None => (*curr.as_ref().unwrap().borrow_mut()).lchild = None,
        };
    }

    #[inline]
    pub fn right_change(curr: &Option<Rc<RefCell<TreeNode<T>>>>, value: Option<T>) {
        match value {
            Some(value) => (*curr.as_ref().unwrap().borrow_mut()).rchild = TreeNode::new(value),
            None => (*curr.as_ref().unwrap().borrow_mut()).rchild = None,
        };
    }
    
    pub fn make_tree(
        &mut self,
        lchild: &Option<Rc<RefCell<TreeNode<T>>>>,
        rchild: &Option<Rc<RefCell<TreeNode<T>>>>,
    ) {
        (*self).lchild = lchild.clone();
        (*self).rchild = rchild.clone();
    }
}

impl<T: Clone> TreeNode<T> {
    #[inline]
    pub fn value(curr: &Option<Rc<RefCell<TreeNode<T>>>>) -> T {
        (*curr.as_ref().unwrap().borrow()).data.clone()
    }
}
