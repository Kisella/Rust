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
- 使用`BinaryHeap::from([v1, v2, ...])`直接从数组中创建最大堆
```rust
    let mut heap3: BinaryHeap<i32> = BinaryHeap::from([3, 1, 4, 1, 5, 9]);
```
&nbsp;
## 2 查看堆顶元素
- `peek()`方法获取堆顶元素的共享引用
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let max = heap.peek(); // 获取最大元素，但不移除它
    assert_eq!(max, Some(&9)); // 9 是最大元素
```
- `peek_mut()`方法获取栈顶元素的可变引用，允许修改堆顶元素
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 10; // 修改最大元素的值, 不改变堆的结构
    drop(max);
    assert_eq!(heap.peek(), Some(&10)); // 10 是新的最大元素
```
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    let mut max = heap.peek_mut().unwrap(); 
    *max = 2; // 修改最大元素的值，堆结构被改变
    drop(max);
    assert_eq!(heap.peek(), Some(&5)); // 5 是新的最大元素
```
&nbsp;
## 3 元素的插入和删除
- `push()`方法插入新元素
- 插入后自动执行上浮操作，将新元素移动到合适位置
```rust
    let heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    heap.push(10);
    assert_eq!(heap.peek(), Some(&10)); // 10 是最大元素
```
- `pop()`方法弹出堆顶元素
- 弹出后自动执行下沉操作，恢复堆结构
```rust
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    assert_eq!(heap.pop(), Some(9));
```
&nbsp;
## 4 堆排序
- `into_sorted_vec()`方法将堆转换为已排序的向量
```rust
    let mut heap1 = vec![3, 1, 4, 1, 5, 9].into_iter().collect::<BinaryHeap<i32>>();
    let vec1 = heap1.into_sorted_vec(); // 将堆转换为一个排序的向量
    assert_eq!(vec1, vec![1, 1, 3, 4, 5, 9]);
```
## 5 堆合并
- `append(other: &mut BinaryHeap<T>)`方法将另一个堆的所有元素合并到当前堆
```rust
    let mut heap1 = vec![3, 1, 4, 1, 5, 9].into_iter().collect::<BinaryHeap<i32>>();
    let mut heap2 = BinaryHeap::from(vec![2, 7, 1, 8, 2, 8]);
    heap1.append(&mut heap2);
    assert_eq!(heap1.len(), 12); // heap1 现在包含了 heap2 的元素
    assert_eq!(heap2.len(), 0); // heap2 已经被清空
    assert_eq!(heap1.into_sorted_vec(), vec![1, 1, 1, 2, 2, 3, 4, 5, 7, 8, 8, 9]);
```
&nbsp;
## 6 堆清空
- `clear()`方法将堆清空

## 7 最小堆
- `BinaryHeap`默认是最大堆，可使用`Reverse`包装实现最小堆
- 使用`.0`获取`Reverse`包裹的值
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn main() {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::from(vec![
        Reverse(3),
        Reverse(1),
        Reverse(4),
        Reverse(1),
        Reverse(5),
        Reverse(9),
    ]);
    let min: Option<Reverse<i32>> = min_heap.pop(); 
    assert_eq!(min, Some(Reverse(1)));
    assert_eq!(min.unwrap().0, 1);
}
```
&nbsp;
## 8 算法复杂度分析
| 操作     | 时间复杂度 | 说明                        |
| -------- | ---------- | --------------------------- |
| `push`   | O(log n)   | 插入元素并上浮调整          |
| `pop`    | O(log n)   | 移除堆顶并下沉调整          |
| `peek`   | O(1)       | 查看堆顶元素              |
| `peek_mut`   | O(1) or O(log n)      | 修改堆顶元素              |
| `new`    | O(1)       | 创建空堆                    |
| `clear`  | O(1)       | 重置堆                      |
| `append` | O(m log n) | 合并两个堆（m为另一堆大小） |
| `into_sorted_vec` | O(n log n) | 堆排序 |

&nbsp;


