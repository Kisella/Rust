use std::collections::BTreeSet;
fn main() {
    // 数组转化为集合时自动去重
    let mut set = BTreeSet::from([3, 1, 4, 1, 5, 9, 2, 6]);

    for num in set.iter() {
        print!("{} ", num); // 输出: 1 2 3 4 5 6 9
    }
    assert_eq!(set.iter().max(), Some(&9));
    assert_eq!(set.iter().min(), Some(&1));
}