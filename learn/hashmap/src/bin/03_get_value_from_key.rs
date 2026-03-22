fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::from([("blue", 10), ("yellow", 50), ("red", 20)]);    

    assert_eq!(scores.get("green"), None); // 获取不存在的键返回 None
    let blue_score = scores.get("blue").unwrap();
    let red_score = scores.get_mut("red").unwrap();

    *red_score = 30; // 修改 red 对应的值
    assert_eq!(scores.get("red"), Some(&30));

    let yellow_score = scores.remove("yellow").unwrap();
    assert_eq!(yellow_score, 50);
    assert_eq!(scores.remove("yellow"), None);

}

