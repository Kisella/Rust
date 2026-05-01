# BtreeSet
BTreeSet 是 Rust 标准库中提供的一个基于 B 树（B-Tree）实现的有序集合。它存储唯一的元素，并保证元素始终按键的自然顺序或自定义比较器进行排列。
&nbsp;
## 1 类型与引入
- `BTreeSet<T>`
- 对于同一个`BTreeSet`, 元素类型`T`必须相同
- 使用`use std::collections::BTreeSet`引入路径
&nbsp;
## 2 创建
- 使用`BTreeSet::new()`创建空`BtreeSet`
- 使用`BTreeSet::from([v1, v2, ...])`创建包含多个元素的`BtreeSet`
- 也可通过迭代器进行创建
```rust
    use std::collections::BTreeSet;
    let mut set: BTreeSet<i32> = BTreeSet::new();
    // 数组转化为集合时自动去重
    let mut numbers = BTreeSet::from([3, 1, 4, 1, 5, 9, 2, 6]);
    let numbers: BTreeSet<i32> = (1..=5).collect();
```
&nbsp;
## 3 插入元素
- 使用`insert(V)`方法插入元素
- 使用`len()`方法获取元素数量
- 使用`is_empty()`方法检查是否为空
```rust
    let mut set = BTreeSet::new();
    assert_eq!(set.is_empty(), true);
    set.insert(1);
    set.insert(2);
    assert_eq!(set.len(), 2);
```
&nbsp;
## 4 查询与删除
- 使用`contains(&V)`方法检查元素是否存在
```rust
    let mut set = BTreeSet::from([1, 2, 3, 4, 5]);
    assert_eq!(set.contains(&5), true);
    assert_eq!(set.contains(&9), false);
```
&nbsp;
- 使用`remove(&V)`方法删除元素
```rust
    assert_eq!(set.contains(&5), true);
    set.remove(&5);
    assert_eq!(set.contains(&5), false);
```
&nbsp;
## 5 遍历
- 使用`iter()`方法默认进行升序迭代
- `set.iter().max()`获取最大元素
- `set.iter().min()`获取最小元素
```rust
    // 数组转化为集合时自动去重
    let mut set = BTreeSet::from([3, 1, 4, 1, 5, 9, 2, 6]);

    for num in set.iter() {
        print!("{} ", num); // 输出: 1 2 3 4 5 6 9
    }
    assert_eq!(set.iter().max(), Some(&9));
    assert_eq!(set.iter().min(), Some(&1));
```
&nbsp;
## 6 范围查询
由于`BTreeSet`保证了元素的有序性，故支持范围查询，这是`BTreeSet`最强大的特性之一
- 使用`range(Range)`方法，进行范围查找
```rust
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

fn main() {
    let mut set = (1..=10).collect::<BTreeSet<_>>();

    // 包含端点的范围 [3, 7]
    let range1: Vec<_> = set.range(3..=7).collect();
    assert_eq!(range1, vec![&3, &4, &5, &6, &7]);

    // 排除端点的范围 (2, 8)
    let range2: Vec<_> = set.range((Excluded(2), Excluded(8))).collect();
    assert_eq!(range2, vec![&3, &4, &5, &6, &7]);

    // 从某个值开始到结尾 [5, ∞)
    let range3: Vec<_> = set.range(5..).collect();
    assert_eq!(range3, vec![&5, &6, &7, &8, &9, &10]);

    // 从开头到某个值 (-∞, 4]
    let range4: Vec<_> = set.range(..=4).collect();
    assert_eq!(range4, vec![&1, &2, &3, &4]);

    // 整个范围
    let range5: Vec<_> = set.range(..).collect();
    assert_eq!(range5.len(), 10);
}
```
&nbsp;
## 7 集合运算
`BTreeSet`支持丰富的集合运算：
- 并集 (Union)
- 交集 (Intersection)
- 差集 (Difference)
- 对称差集 (Symmetric Difference)
- 子集检查 (Subset)
- 超集检查 (Superset)
- 不相交检查 (Disjoint)
### 7.1 并集
- 取两个集合的元素组成新集合
- 使用`set1.union(&set2)`进行并集运算
- 也可使用运算符 `|` 进行并集运算
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].into();

    let union: BTreeSet<i32> = set1.union(&set2).copied().collect();
    assert_eq!(union, [1, 2, 3, 4, 5, 6].into());
    let union = &set1 | &set2;
    assert_eq!(union, [1, 2, 3, 4, 5, 6].into());
```
&nbsp;
### 7.2 交集
- 取两个集合的共有元素组成新集合
- 使用`set1.intersection(&set2)`进行交集运算
- 也可使用运算符 `&` 进行交集运算
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].into();

    let intersection: BTreeSet<i32> = set1.intersection(&set2).copied().collect();
    assert_eq!(intersection, [3, 4].into());
    let intersection = &set1 & &set2;
    assert_eq!(intersection, [3, 4].into());
```
&nbsp;
### 7.3 差集
- 被减集合减去两个集合的共有元素组成新集合
- 使用`set1.difference(&set2)`进行差集运算
- 也可使用运算符 `-` 进行差集运算
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].into();

    let difference: BTreeSet<i32> = set1.difference(&set2).copied().collect();
    assert_eq!(difference, [1, 2].into());
    let difference = &set1 - &set2;
    assert_eq!(difference, [1, 2].into());
```
&nbsp;
### 7.4 对称差集
- 取两个集合的非共有元素组成新集合
- 使用`set1.symmetric_difference(&set2)`进行对称差集运算
- 也可使用运算符 `^` 进行对称差集运算
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].into();

    let symmetric_diff: BTreeSet<i32> = set1.symmetric_difference(&set2).copied().collect();
    assert_eq!(symmetric_diff, [1, 2, 5, 6].into());
    let symmetric_diff = &set1 ^ &set2;
    assert_eq!(symmetric_diff, [1, 2, 5, 6].into());
```
&nbsp;
### 7.5 子/超集检查
- 检查一个集合(超集)是否包含了另一个集合（子集）的所有元素
- `small_set.is_subset(&set1)`进行子集检查
- `set1.is_superset(&small_set)`进行超集检查
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let small_set: BTreeSet<i32> = [1, 2].into();
    assert!(small_set.is_subset(&set1));
    assert!(set1.is_superset(&small_set));
```
&nbsp;
### 7.6 不相交检查
- `set1.is_disjoint(&disjoint_set)`进行不相交检查
```rust
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let disjoint_set: BTreeSet<i32> = [7, 8].into();
    assert!(set1.is_disjoint(&disjoint_set));
```
&nbsp;
### 8 Ord Trait
- `BtreeSet`的元素类型必须实现了`Ord` trait, 用于元素之间进行比较
- 实现`Ord`，需先实现`PartialEq`、`Eq` 和 `PartialOrd`
```
use std::collections::BTreeSet;
use std::cmp::Ordering;
struct Student {
    name: String,
    id: u32,
    age: u32,
    class: String,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Student {}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    let student1 = Student {
        name: "Alice".to_string(),
        id: 1001,
        age: 20,
        class: "Math".to_string(),
    };
    let set = BTreeSet::from([(student1)]);
}
```
&nbsp;
## 9 `HashSet` VS `BtreeSet`
| 特性     | BTreeSet             | HashSet                      |
| -------- | -------------------- | ---------------------------- |
| 顺序     | 有序（按键排序）     | 无序                         |
| 查找时间 | O(log n)             | 平均 O(1)，最坏 O(n)         |
| 内存使用 | 较少（无哈希表开销） | 较多（需要额外空间避免冲突） |
| 范围查询 | ✅ 高效支持           | ❌ 不支持                     |
| 元素要求 | `Ord` trait          | `Hash + Eq` traits           |
| 迭代顺序 | 稳定（排序顺序）     | 不稳定（依赖哈希）           |
| 集合运算 | ✅ 高效（有序遍历）   | ⚠️ 较慢（需要额外排序）       |

