/*
 * Iterator Trait 和 next方法
 * 所有的迭代器都实现了标准库中的Iterator特性
 * 实现Iterator特性只要求实现一个next方法
 * next方法每次返回迭代器中的一个item包裹在Some中，若迭代结束则返回None
 */ 
#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();    //  调用next会消耗迭代器，故需要设定为可变的

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}