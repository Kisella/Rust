use std::fmt::Display;

/*
 * case 47
 * 使用Trait Bound来有条件地为带有泛型参数T的类型实现方法
 */
struct Pair<T> {
    x: T,
    y: T,
}

// 为无限制的泛型T实现方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

// 为实现有Display和PartialOrd特质的泛型T实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn com_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largesr member is y = {}", self.y);
        }
    }
}

// Rust要求泛型参数只能Trait约束，而不能受具体类型的约束，这是设计上的选择，旨在保持类型系统的统一和代码的抽象能力。而直接为具体类型实现方法（如`impl Pair<f32>`）是正确且推荐的做法，类似impl<T: f32> Pair<T> 这样的写法是不被允许的
impl Pair<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}

fn main() {
    println!("case 46");
    let pair1 = Pair::new((1, 2), (3, 4));
    let pair2 = Pair::new(8, 3);
    let pair3 = Pair::new(9.5, 3.14);

    pair1.x();
    pair2.x();
    pair3.x();  //  所有的pair实例都有x方法

 // pair1.com_display();   // pair1没有com_display方法
    pair2.com_display();
    pair3.com_display();
    println!("pair1 is not com_display method.\n");

 // pair1.y();
 // pair2.y();   //  pair1和pair2没有y方法
    pair3.y();
    println!("pair1 and par2 is not y method.");
}
