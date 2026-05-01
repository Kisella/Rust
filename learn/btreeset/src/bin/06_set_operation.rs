use std::collections::BTreeSet;
fn main() {
    let set1: BTreeSet<i32> = [1, 2, 3, 4].into();
    let set2: BTreeSet<i32> = [3, 4, 5, 6].into();

    let union: BTreeSet<i32> = set1.union(&set2).copied().collect();
    assert_eq!(union, [1, 2, 3, 4, 5, 6].into());
    let union = &set1 | &set2;
    assert_eq!(union, [1, 2, 3, 4, 5, 6].into());

    // 交集 (Intersection)
    let intersection: BTreeSet<i32> = set1.intersection(&set2).copied().collect();
    assert_eq!(intersection, [3, 4].into());
    let intersection = &set1 & &set2;
    assert_eq!(intersection, [3, 4].into());

    // 差集 (Difference)
    let difference: BTreeSet<i32> = set1.difference(&set2).copied().collect();
    assert_eq!(difference, [1, 2].into());
    let difference = &set1 - &set2;
    assert_eq!(difference, [1, 2].into());

    // 对称差集 (Symmetric Difference)
    let symmetric_diff: BTreeSet<i32> = set1.symmetric_difference(&set2).copied().collect();
    assert_eq!(symmetric_diff, [1, 2, 5, 6].into());
    let symmetric_diff = &set1 ^ &set2;
    assert_eq!(symmetric_diff, [1, 2, 5, 6].into());

    // 子集检查 (Subset)
    let small_set: BTreeSet<i32> = [1, 2].into();
    assert!(small_set.is_subset(&set1));

    // 超集检查 (Superset)
    assert!(set1.is_superset(&small_set));

    // 不相交检查 (Disjoint)
    let disjoint_set: BTreeSet<i32> = [7, 8].into();
    assert!(set1.is_disjoint(&disjoint_set));
}