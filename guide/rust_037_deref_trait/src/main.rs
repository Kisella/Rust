/*
 * 实现Deref Trait 使我们可以自定义解引用运算符(*)的行为。通过实现Deref, 智能指针就可像常规引用一样来处理
 */

use std::ops::Deref;
//  定义自己的智能指针
struct MyBox<T>(T);     //   元组结构体
impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0    //   将元组中的第一个元素返回, 由于返回的是引用，所以就可以使用解引用来获得这个值
    }
}

// 标准库中的Deref trait要求我们实现一个deref方法，该方法借用self, 返回一个指向内部数据的引用

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);    // Compiling error, 没有整数和整数的引用进行比较的功能

    let x = 5;
    let y = Box::new(x);    //  y从原来的&x变成了Box<x>, 断言效果相同。通过实现Deref, 智能指针就可像常规引用一样来处理
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);  // MyBox类型实现Deref特性(Trait)后, 就可以使用解引用符号了
    // *y <=> *(y.deref()), Rust会将*y隐式地展开为*(y.deref())的形式 (隐式解引用转化)
}
