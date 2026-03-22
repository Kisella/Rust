fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert("blue", 10);
    assert_eq!(scores.get("blue"), Some(&10));
    scores.insert("blue", 25); // 覆盖之前的值
    assert_eq!(scores.get("blue"), Some(&25));

    let blue_score = scores.entry("blue").or_insert(50); // 已经存在 "blue"，所以不修改值
    assert_eq!(blue_score, &25);
    *blue_score = 30; // 修改 blue 对应的值
    assert_eq!(scores.get("blue"), Some(&30));


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
}