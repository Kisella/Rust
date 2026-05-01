fn main() {
    use std::collections::BTreeSet;
    let mut set: BTreeSet<i32> = BTreeSet::new();
    // 数组转化为集合时自动去重
    let mut numbers = BTreeSet::from([3, 1, 4, 1, 5, 9, 2, 6]);
    let numbers: BTreeSet<i32> = (1..=5).collect();
}