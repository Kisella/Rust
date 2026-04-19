use std::collections::BTreeMap;
fn main() {
    let mut scores = BTreeMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);    

    assert_eq!(scores.get("green"), None); // 获取不存在的键返回 None
    let blue_score = scores.get("blue").unwrap();
    let red_score = scores.get_mut("red").unwrap();

    *red_score = 30; // 修改 red 对应的值
    assert_eq!(scores.get("red"), Some(&30));


    let mut scores = BTreeMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);
    assert_eq!(scores["red"], 20);  // ✅ BTreeMap实现了Index特征
    // scores["blue"] = 25; //         ❌ BTreeMap未实现IndexMut特征
}