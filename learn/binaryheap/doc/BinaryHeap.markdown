# BinaryHeap
Rust中的`BinaryHeap`是标准库提供的**二叉最大堆**实现，位于`std::collections`模块，主要实现优先队列结构，具有高效的插入、删除、访问最大值能力。
&nbsp;
## 1 创建
- 使用`BinaryHeap::new()`创建空的最大堆
- 使用`BinaryHeap::with_capacity(usize)`指定初始容量的最大堆
- 元素类型`T`需实现了`Ord`特质
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::with_capacity(10); // 指定初始容量
}
```
- 使用`BinaryHeap::from(Vec<T>)`直接从向量中创建最大堆
```rust
    let mut heap3: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
```
&nbsp;
## 2 查看堆顶元素
- `peek()`方法获取堆顶元素的共享引用
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let max = heap.peek(); // 获取最大元素，但不移除它
    assert_eq!(max, Some(&9)); // 9 是最大元素
```
- `peek_mut()`方法获取栈顶元素的可变引用，允许在不破坏堆性质情况下修改堆顶元素
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 10; // ✅ 修改最大元素的值, 不改变堆的结构
    drop(max);
    assert_eq!(heap.peek(), Some(&10)); // 10 是新的最大元素
```
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 2; // ❌ 修改最大元素的值，堆结构被破坏，行为不可控
```


## 3 元素的插入和删除
- 使用`push`方法插入新元素
- 插入后自动执行上浮操作，将新元素移动到合适位置