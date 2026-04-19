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