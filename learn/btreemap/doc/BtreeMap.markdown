# BtreeMap
`BTreeMap` 是 Rust 标准库中提供的一个基于 B 树（B-Tree）实现的有序键值对集合。它与 `HashMap` 不同，保证了键的有序性，这使得它在需要范围查询、有序遍历等场景中非常有用。
&nbsp;
## 1 类型与引入
- `BTreeMap<K, V>`, 其中`K`为键类型，`V`为值类型
- 对于同一个`BTreeMap`, `K`类型必须相同，`V`类型也必须相同
- 使用`use std::collections::BTreeMap`引入路径
&nbsp;
## 2 创建
- 使用`BTreeMap::new()`创建空`BtreeMap`
- 使用`BTreeMap::from([(k1, v1), (k2, v2), ...])`创建包含多个键值对的`BtreeMap`
```rust
use std::collections::BTreeMap;
fn main() {
    let mut map1: BTreeMap<String, i32> = BTreeMap::new();
    let mut map2 = BTreeMap::from([
        ("apple", 3),
        ("banana", 2),
        ("orange", 5),
    ]);
}
```
- 可通过迭代器创建`BTreeMap`
```rust
    let teams = vec!["blue", "red"];
    let inital_scores = vec![10, 50];
    let scores: BTreeMap<_, _> = teams.into_iter().zip(inital_scores.into_iter()).collect();    //  使用 zip 方法将两个向量组合成一个迭代器，然后使用 collect 方法将其转换为 BTreeMap
```
&nbsp;
## 3 查询与删除
- 使用`get(&K)`方法获取键对应值的共享引用
- 使用`get_mut(&K)`方法获取键对应值的可变引用
- 获取值失败时返回`None`
```rust
use std::collections::BTreeMap;
fn main() {
    let mut scores = BTreeMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);    
    assert_eq!(scores.get("green"), None); // 获取不存在的键返回 None
    let blue_score = scores.get("blue").unwrap();
    let red_score = scores.get_mut("red").unwrap();
    *red_score = 30; // 修改 red 对应的值
    assert_eq!(scores.get("red"), Some(&30));
}
```
- `BTreeMap`自动实现了`Index`特征，可以使用键作索引获取值
```rust
    let mut scores = BTreeMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);
    assert_eq!(scores["red"], 20);  // ✅ BTreeMap实现了Index特征
    // scores["blue"] = 25; //         ❌ BTreeMap未实现IndexMut特征
```
&nbsp;
## 4 插入与更新
- 使用`insert(K, V)`方法插入键值对
- 当键已存在时，会覆盖原有的值
```rust
    scores.insert("blue", 10);
    assert_eq!(scores.get("blue"), Some(&10));
    scores.insert("blue", 25); // 覆盖之前的值
    assert_eq!(scores.get("blue"), Some(&25));
```
&nbsp;
- 使用`entry(K).or_insert(V)`, 仅在键不存在时插入
- `or_inset()`方法返回值的可变引用，可以此来更新已有值
```rust
    scores.insert("blue", 25);
    let blue_score = scores.entry("blue").or_insert(50); // 已经存在键"blue"，所以不修改值
    assert_eq!(blue_score, &25);
    *blue_score = 30; // 修改 blue 对应的值
    assert_eq!(scores.get("blue"), Some(&30));
```
&nbsp;
- 使用`entry(K).and_modify(|&mut V| {}).or_insert(V)`进行流程化处理
- 使用`*btreemap.get_mut(&K).unwrap()`对值进行单点更新
```rust
    let vec = vec![("blue", 7), ("yellow", 13)];
    let mut btreemap = vec.into_iter().collect::<BTreeMap<_,_>>();

    // 若键值对存在则值加1，若键值对不存在则插入新键值对并初始化值为1
    btreemap.entry("blue").and_modify(|value| *value += 1).or_insert(1);
    btreemap.entry("yellow").and_modify(|value| *value += 1).or_insert(1);
    btreemap.entry("green").and_modify(|value| *value += 1).or_insert(1);
    assert_eq!(btreemap.get("blue"), Some(&8));
    assert_eq!(btreemap.get("yellow"), Some(&14));
    assert_eq!(btreemap.get("green"), Some(&1));

    *btreemap.get_mut("green").unwrap() = 19;
    assert_eq!(btreemap.get("green"), Some(&19));
```
&nbsp;
## 5 遍历
- for循环中使用`btreemap.iter()`或`&btreemap`来遍历键值对
- 使用`btreemap.keys()`遍历键
- 使用`btreemap.values()`遍历值
- `BTreeMap`的迭代是**有序**的，默认按键的升序迭代
```rust
    let vec = vec![
        (2, "value2".to_string()),
        (1, "value1".to_string()),
        (4, "value4".to_string()),
        (3, "value3".to_string()),
    ];
    let btreemap = vec.into_iter().collect::<BTreeMap<_, _>>();
    for (key, value) in &btreemap {
        println!("{}: {:?}", key, value);
    }
    println!("{btreemap:?}");
```
&nbsp;
## 6 范围查找
由于`BTreeMap`保证了键的有序性，故支持范围查询，这是`BTreeMap`最强大的特性之一
- 使用`range(Range)`方法，进行范围查找
```rust
use std::collections::BTreeMap;
use std::ops::Bound::Excluded;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")]);

    // 包含端点的范围 [2, 4]
    let range1: Vec<_> = map.range(2..=4).collect();
    assert_eq!(range1, [(&2, &"b"), (&3, &"c"), (&4, &"d")]);

    // 排除端点的范围 (1, 4)
    let range2: Vec<_> = map.range((Excluded(1), Excluded(4))).collect();
    assert_eq!(range2, [(&2, &"b"), (&3, &"c")]);

    // 从某个值开始到结尾 [3, ∞)
    let range3: Vec<_> = map.range(3..).collect();
    assert_eq!(range3, [(&3, &"c"), (&4, &"d"), (&5, &"e")]);

    // 从开头到某个值 (-∞, 3]
    let range4: Vec<_> = map.range(..=3).collect();
    assert_eq!(range4, [(&1, &"a"), (&2, &"b"), (&3, &"c")]);

    // 整个范围
    let range5: Vec<_> = map.range(..).collect();
    assert_eq!(range5.len(), 5);
}
```
&nbsp;
## 7 分割与合并
- 使用`split_off(&K)`方法分割`BtreeMap`
- 原表变为小于等于某个键的所有元素
```rust
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    let split_off = map.split_off(&3);
    assert_eq!(map, BTreeMap::from([(1, "a"), (2, "b")]));
    assert_eq!(split_off, BTreeMap::from([(3, "c"), (4, "d")]));
```
&nbsp;
- 使用`append(&mut btreemap)`方法将另一个`BtreeMap`的内容合并进来
- 被合并的`BtreeMap`变为空表
```rust
    let mut map1 = BTreeMap::from([(1, "a"), (2, "b")]);
    let mut map2 = BTreeMap::from([(3, "c"), (4, "d")]);
    map1.append(&mut map2);
    assert_eq!(map1, BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]));
    assert_eq!(map2.len(), 0);
```
&nbsp;
## 8 `HashMap` VS `BtreeMap`
| 特性     | BTreeMap             | HashMap                       |
| -------- | -------------------- | ----------------------------  |
| 顺序     | 有序                 | 无序                           |
| 查找时间 | O(log n)             | 平均 O(1)，最坏 O(n)            |
| 内存使用 | 较少（无哈希表开销） | 较多（需要额外空间避免冲突）       |
| 范围查询 | ✅ 高效支持           | ❌ 不支持                     |
| 键要求   | `Ord` trait          | `Hash + Eq` traits            |
| 缓存性能 | 较好（B 树设计）     | 一般（哈希可能导致缓存未命中）     |

