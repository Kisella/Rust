use std::collections::HashMap;
// HashMap的大小是可变的
// HashMap的每一个键K只能对应一个值V
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // 如果使用insert方法向HashMap插入一个键值对，然后插入同样的key但是value不同，那么原来的value就会被覆盖掉

    // entry方法插入键值对
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 键Blue已存在，or_insert不影响
    println!("{:?}", scores);
    // entry方法, 参数为K, 可检查指定的Key是否存在, 并返回Entry枚举类型
    // or_insert方法, 参数为V, 若entry返回的是VacantEntry变体(代表键不存在), 则为hashmap插入一个新键值对。若键已存在，则or_insert仅返回一个该K对应V的可变引用，不会新增键值对，也不改变原有的值

    // 基于现有的V来更新V
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {   //  按空格对text进行分割并遍历
        let count = map.entry(word).or_insert(0);   //  调用or_insert方法初始化键值对, 并返回对应V的可变引用
        *count += 1;    //  使用解引用符号(*)对现有的V进行修改
    }
    println!("{:#?}", map);
}
