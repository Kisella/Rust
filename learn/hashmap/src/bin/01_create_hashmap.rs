
fn main() {
    use std::collections::HashMap;
    let mut scores: HashMap<&str, i32> = HashMap::new();    //  声明变量时显式指定键类型和值类型

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);    //  键和值的类型由 insert 方法的参数推断得出

    let mut scores = HashMap::from([("Blue", 10), ("Yellow", 50), ("red", 20)]);    //  使用 from 函数创建 HashMap，键值对作为元组传递

    let teams = vec!["blue", "red"];
    let inital_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();    //  使用 zip 方法将两个向量组合成一个迭代器，然后使用 collect 方法将其转换为 HashMap

    let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
}