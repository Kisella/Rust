use std::collections::BTreeMap;
fn main() {
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
}
