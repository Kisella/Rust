use std::collections::BTreeMap;
fn main() {
    let mut map1: BTreeMap<String, i32> = BTreeMap::new();
    let mut map2 = BTreeMap::from([
        ("apple", 3),
        ("banana", 2),
        ("orange", 5),
    ]);

    let teams = vec!["blue", "red"];
    let inital_scores = vec![10, 50];
    let scores: BTreeMap<_, _> = teams.into_iter().zip(inital_scores.into_iter()).collect();    //  使用 zip 方法将两个向量组合成一个迭代器，然后使用 collect 方法将其转换为 BTreeMap
}