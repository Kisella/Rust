use std::collections::BinaryHeap;
fn main() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    heap.push(10);
    assert_eq!(heap.peek(), Some(&10)); // 10 是最大元素

    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    assert_eq!(heap.pop(), Some(9));
}