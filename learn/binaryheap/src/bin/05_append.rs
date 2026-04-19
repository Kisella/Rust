use std::collections::BinaryHeap;
fn main() {
    let heap1 = vec![3, 1, 4, 1, 5, 9].into_iter().collect::<BinaryHeap<i32>>();
    let vec1 = heap1.into_sorted_vec(); // 将堆转换为一个排序的向量
    assert_eq!(vec1, vec![1, 1, 3, 4, 5, 9]);


    let mut heap1 = vec![3, 1, 4, 1, 5, 9].into_iter().collect::<BinaryHeap<i32>>();
    let mut heap2 = BinaryHeap::from(vec![2, 7, 1, 8, 2, 8]);
    heap1.append(&mut heap2); // 将 heap2 的元素移动到 heap1 中
    assert_eq!(heap1.len(), 12); // heap1 现在包含了 heap2 的元素
    assert_eq!(heap2.len(), 0); // heap2 已经被清空
    assert_eq!(heap1.into_sorted_vec(), vec![1, 1, 1, 2, 2, 3, 4, 5, 7, 8, 8, 9]);
}