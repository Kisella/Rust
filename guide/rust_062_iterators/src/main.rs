/*
 * iter() --> 获得项目不可变引用的迭代器
 * into_iter() --> 能获得项目的所有权，并返回具有所有权的值
 * iter_mut() --> 获得项目可变引用的迭代器
 */
fn main() {
    /*
     * Interator trait 在标准库中有许多默认实现的方法，其中一些方法在定义时调用了next方法，这些调用了next方法的方法被称为“消耗性适配器”。一个典型例子是sum方法。
     */
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); //  sum会消耗掉迭代器
    assert_eq!(total, 6);
    // println!("{v1_iter:?}");     //  v1_iter已经被消耗掉，不能再继续使用

    /*
     * “迭代器适配器”是定义在Iterator trait上的方法。它不会消耗原始迭代器，而会通过修改原始迭代器的某些方面来产生新的迭代器。一个典型例子是map方法。
     */
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    let v = vec![1, 2, 3, 4];
    // filter 接收的是迭代器元素的引用, 所以需要解引用一次才能访问原始值
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    // map 接收的是迭代器元素本身，所以不需要解引用就能访问原始值。map返回迭代器元素类型为i32, filter接收迭代器元素的引用即为&i32
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);
}
