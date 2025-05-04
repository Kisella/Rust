/*
 * 堆结构是一个完全二叉树，可用Vec进行存储
 * 当前节点 i
 * 左孩子 2*i+1
 * 右孩子 2*i+2
 * 父节点 (i+1)/2 - 1
 * 
 */ 

pub struct PriorityQueue<T> {
    pub heap: Vec<T>,
}
