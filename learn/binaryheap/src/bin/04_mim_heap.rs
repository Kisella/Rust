use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn main() {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::from(vec![
        Reverse(3),
        Reverse(1),
        Reverse(4),
        Reverse(1),
        Reverse(5),
        Reverse(9),
    ]);
    let min: Option<Reverse<i32>> = min_heap.pop(); 
    assert_eq!(min, Some(Reverse(1)));
    assert_eq!(min.unwrap().0, 1);
}