use std::collections::BinaryHeap;

fn main() {
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::with_capacity(10); // 指定初始容量

    let mut heap3: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]); // 从一个向量创建二叉堆
}