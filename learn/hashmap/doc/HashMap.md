# HashMap
`HashMap`是标准库(`std::collections`)提供的一个非常重要的数据结构，用于实现键值对映射。它基于哈希表实现，提供平均时间复杂度为 O(1) 的插入、查找和删除操作。
&nbsp;
## 1 类型与引入
- `HashMap<K, V>`, 其中`K`为键类型，`V`为值类型
- 对于同一个`HashMap`, `K`类型必须相同，`V`类型也必须相同
- 使用`use std::collections::HashMap`引入路径
&nbsp;
## 2 创建
- 使用`HashMap::new()`函数创建空`HashMap`
- `<K, V>`泛型参数类型必须被唯一确定，可以在声明变量时显式指定，也可通过后续插入键值对使编译器能推导类型
```rust
    use std::collections::HashMap;
    let mut scores: HashMap<&str, i32> = HashMap::new(); // 声明变量时显式指定键类型和值类型

    let mut scores = HashMap::new();
    scores.insert("Blue", 10); // 键和值的类型由 insert 方法的参数推断得出
```
&nbsp;
- 使用`HashMap::from([(k1, v1), (k2, v2), ...])`创建包含多个键值对的`HashMap`
- 键值对通过元组传递，多个键值对组成数组，作为`HashMap::from()`函数的参数
```rust
    let mut scores = HashMap::from([("Blue", 10), ("Yellow", 50), ("red", 20)]);
```
&nbsp;
- 也可通过迭代器创建`HashMap`
```rust
    let teams = vec!["blue", "red"];
    let inital_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();    //  使用 zip 方法将两个向量组合成一个迭代器，然后使用 collect 方法将其转换为 HashMap
```
&nbsp;
- 使用`HashMap::with_capacity(size)`预分配容量，避免频繁扩容
```rust
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
```
&nbsp;
## 3 查询与删除
- 使用`hashmap.get(&K)`方法获取键对应值的共享引用
- 使用`hashmap.get_mut(&K)`方法获取键对应值的可变引用
- 获取值失败时返回`None`
```rust
    let mut scores = HashMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);    

    assert_eq!(scores.get("green"), None); // 获取不存在的键返回 None
    let blue_score = scores.get("blue").unwrap();
    let red_score = scores.get_mut("red").unwrap();

    *red_score = 30; // 修改 red 对应的值
    assert_eq!(scores.get("red"), Some(&30));
```
- `HashMap`自动实现了`Index`特征，可以使用键作索引获取值
```rust
    let mut scores = HashMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);
    assert_eq!(scores["red"], 20);  // ✅ HashMap实现了Index特征
    // scores["blue"] = 25; //         ❌ HashMap未实现IndexMut特征
```
&nbsp;
- 使用`hashmap.remove(&K)`方法获取键对应值的所有权，同时删除原有键值对
- 删除键值对失败时，返回`None`
```rust
    let yellow_score = scores.remove("yellow").unwrap();
    assert_eq!(yellow_score, 50);
    assert_eq!(scores.remove("yellow"), None)
```
&nbsp;
## 4 插入与更新
- 使用`hashmap.insert(K, V)`方法插入键值对
- 当键已存在时，会覆盖原有的值
```rust
    scores.insert("blue", 10);
    assert_eq!(scores.get("blue"), Some(&10));
    scores.insert("blue", 25); // 覆盖之前的值
    assert_eq!(scores.get("blue"), Some(&25));
```
&nbsp;
- 使用`hashmap.entry(K).or_insert(V)`, 仅在键不存在时插入
- `or_inset()`方法返回值的可变引用，可以此来更新已有值
```rust
    scores.insert("blue", 25);
    let blue_score = scores.entry("blue").or_insert(50); // 已经存在键"blue"，所以不修改值
    assert_eq!(blue_score, &25);
    *blue_score = 30; // 修改 blue 对应的值
    assert_eq!(scores.get("blue"), Some(&30));
```
&nbsp;
- 使用`hashmap.entry(K).and_modify(|&mut V| {}).or_insert(V)`进行流程化处理
- 使用`*hashmap.get_mut(&K).unwrap()`对值进行单点更新
```rust
    let vec = vec![("blue", 7), ("yellow", 13)];
    let mut hashmap = vec.into_iter().collect::<HashMap<_,_>>();

    // 若键值对存在则值加1，若键值对不存在则插入新键值对并初始化值为1
    hashmap.entry("blue").and_modify(|value| *value += 1).or_insert(1);
    hashmap.entry("yellow").and_modify(|value| *value += 1).or_insert(1);
    hashmap.entry("green").and_modify(|value| *value += 1).or_insert(1);
    assert_eq!(hashmap.get("blue"), Some(&8));
    assert_eq!(hashmap.get("yellow"), Some(&14));
    assert_eq!(hashmap.get("green"), Some(&1));

    *hashmap.get_mut("green").unwrap() = 19;
    assert_eq!(hashmap.get("green"), Some(&19));
```
&nbsp;
## 5 遍历
- for循环中使用`hashmap.iter()`或`&hashmap`来遍历键值对
- 使用`hashmap.keys()`遍历键
- 使用`hashmap.values()`遍历值
```rust
    let vec = vec![(1, "value1".to_string()), (2, "value2".to_string())];
    let hashmap = vec.into_iter().collect::<HashMap<_,_>>();
    for (key, value) in &hashmap {
        println!("{}: {:?}", key, value);
    }
    println!("{hashmap:?}");
```
&nbsp;
## 6 Hash Trait
- 键类型必须实现了Hash trait，它用于计算哈希值
-  `i32`, `String`, 元组（若其元素可哈希）等已实现Hash trait
-  使用`#[derive(Hash, PartialEq, Eq)]`为自定义类型自动实现 Hash trait
-  Eq trait的实现必须与 Hash trait一致(若 a == b，则 hash(a) == hash(b))
```rust
    struct Student {
        name: String,
        id: u32,
        age: u32,
        class: String,
    }

    impl Hash for Student {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state); // 只使用 id 字段进行哈希计算
        }
    }

    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id // 只比较 id 字段
        }
    }

    impl Eq for Student {}
```

## 7 小结
- `HashMap<K, V>`, 其中`K`为键类型，`V`为值类型
- `HashMap::from([(k1, v1), (k2, v2), ...])`创建包含多个键值对的`HashMap`
- `hashmap.get_mut(&K)`方法获取值的可变引用
- `hashmap.entry(K).and_modify(|&mut V| {}).or_insert(V)`进行流程化处理