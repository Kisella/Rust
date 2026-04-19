use std::collections::BTreeMap;
use std::ops::Bound::Excluded;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")]);

    // 包含端点的范围 [2, 4]
    let range1: Vec<_> = map.range(2..=4).collect();
    assert_eq!(range1, [(&2, &"b"), (&3, &"c"), (&4, &"d")]);

    // 排除端点的范围 (1, 4)
    let range2: Vec<_> = map.range((Excluded(1), Excluded(4))).collect();
    assert_eq!(range2, [(&2, &"b"), (&3, &"c")]);

    // 从某个值开始到结尾 [3, ∞)
    let range3: Vec<_> = map.range(3..).collect();
    assert_eq!(range3, [(&3, &"c"), (&4, &"d"), (&5, &"e")]);

    // 从开头到某个值 (-∞, 3]
    let range4: Vec<_> = map.range(..=3).collect();
    assert_eq!(range4, [(&1, &"a"), (&2, &"b"), (&3, &"c")]);

    // 整个范围
    let range5: Vec<_> = map.range(..).collect();
    assert_eq!(range5.len(), 5);
}
