use std::collections::BinaryHeap;

fn main() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let max = heap.peek(); // 获取最大元素，但不移除它
    assert_eq!(max, Some(&9)); // 9 是最大元素

    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 10; // 修改最大元素的值, 不改变堆的结构
    drop(max);
    assert_eq!(heap.peek(), Some(&10)); // 10 是新的最大元素

    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 2; // 修改最大元素的值，堆结构被改变
    drop(max);
    assert_eq!(heap.peek(), Some(&5)); // 5 是新的最大元素
}