use std::collections::BTreeMap;
fn main() {
    let mut map1: BTreeMap<String, i32> = BTreeMap::new();
    let mut map2 = BTreeMap::from([
        ("apple", 3),
        ("banana", 2),
        ("orange", 5),
    ]);
}