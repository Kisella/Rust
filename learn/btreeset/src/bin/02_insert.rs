use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    assert_eq!(set.is_empty(), true);
    set.insert(1);
    set.insert(2);
    assert_eq!(set.len(), 2);
}