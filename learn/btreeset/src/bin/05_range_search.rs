use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

fn main() {
    let mut set = (1..=10).collect::<BTreeSet<_>>();

    // 包含端点的范围 [3, 7]
    let range1: Vec<_> = set.range(3..=7).collect();
    assert_eq!(range1, vec![&3, &4, &5, &6, &7]);

    // 排除端点的范围 (2, 8)
    let range2: Vec<_> = set.range((Excluded(2), Excluded(8))).collect();
    assert_eq!(range2, vec![&3, &4, &5, &6, &7]);

    // 从某个值开始到结尾 [5, ∞)
    let range3: Vec<_> = set.range(5..).collect();
    assert_eq!(range3, vec![&5, &6, &7, &8, &9, &10]);

    // 从开头到某个值 (-∞, 4]
    let range4: Vec<_> = set.range(..=4).collect();
    assert_eq!(range4, vec![&1, &2, &3, &4]);

    // 整个范围
    let range5: Vec<_> = set.range(..).collect();
    assert_eq!(range5.len(), 10);
}