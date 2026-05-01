use std::collections::BTreeSet;
fn main() {
    let mut set = BTreeSet::from([1, 2, 3, 4, 5]);
    assert_eq!(set.contains(&5), true);
    assert_eq!(set.contains(&9), false);

    assert_eq!(set.contains(&5), true);
    set.remove(&5);
    assert_eq!(set.contains(&5), false);
}