/*
 *  HashMap<K, V>
 *  hashmap已键值对的形式存储数据，一个键(key)对应一个值(value)
 *  hashmap的数据存储在heap上
 *  在一个hashmap中，所有的键值对必须是同一种类型(形式)
 */
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("{:?}", scores);
    // 创建空hashmap是需要显示声明键值对的类型，或者后续通过insert添加元素，否则rust无法推断出其类型

    //  insert方法添加键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(40, String::from("green"));   //  Compilying error, 在一个hashmap中，所有的键值对必须是同一种类型(形式)
    println!("{:#?}", scores);

    // 使用collect方法创建hanshmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
    // collec方法可以把数据整合成很多种集合类型，包括HashMap, 故需要显示声明返回类型
    println!("{:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:#?}", map);
    // println!("{}: {}", field_name, field_value);    //  Compilying error
    // 对于拥有所有权的值，所有权会移动到HashMap中

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{}: {}", field_name, field_value);
    println!("{:?}", map);
    // 如果将引用插入到hashmap, 则不会发生移动。在hashmap有效期间，被引用的值必须保持有效

    //  get方法访问HashMap中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //  get方法传入键(key)的引用, 返回Option<&V>类型
    match score {
        Some(s) => println!("{s}"),
        None => println!("team not exist"),
    };

    // 遍历HashMap
    for (key, value) in &scores {    //  或使用scores.iter()
        println!("{}: {}", key, value);
    }

    for (index, (key, value)) in scores.iter().enumerate() {
        println!("{}. {}: {}", index, key, value);
    }
}
