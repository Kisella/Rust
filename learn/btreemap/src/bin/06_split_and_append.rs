use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    let split_off = map.split_off(&3);
    assert_eq!(map, BTreeMap::from([(1, "a"), (2, "b")]));
    assert_eq!(split_off, BTreeMap::from([(3, "c"), (4, "d")]));

    let mut map1 = BTreeMap::from([(1, "a"), (2, "b")]);
    let mut map2 = BTreeMap::from([(3, "c"), (4, "d")]);

    map1.append(&mut map2);
    assert_eq!(map1, BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]));
    assert_eq!(map2.len(), 0);
}