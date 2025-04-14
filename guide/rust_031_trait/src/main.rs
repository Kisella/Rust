use std::{fmt::Display, result};
// 使用Trait Bound 有条件的实现方法：
// 在使用了泛型类型参数的impl块上使用Trait Bound, 我们可以有条件的为实现了特定Trait类型来实现方法
struct Pair<T> {
    x:T,
    y:T,
}

impl<T> Pair<T> {   //  为所有的泛型Pair实现方法
    fn new(x:T, y:T) -> Self {
        Self {x,y}
    }
}

impl<T:Display+PartialOrd> Pair<T> {    //  为特定的实现了Display和PartialOrd的TraitBound约束的泛型Pair实现方法
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket实现：可以为实现了某个trait的类型有条件地实现另一个trait
// impl<T:Display> ToString for T {
//     // --snip--
// }   //  为"任何实现了Display这个trait的类型"实现ToString这个Trait, 使其类型也能调用ToString里的方法

fn main() {
    let pair1 = Pair::new(4.5, 3.6);
    let pair2 = Pair::new([4.5], [3.6]);
    pair1.cmp_display();
    // let result = pair2.cmp_display();    //  Compiling error, 数组没有实现Display这个Trait, 故pair2没有cmp_display这个方法

    let s = 3.to_string();      //  标准库中存在对应的blanket实现，使实现了Display的类型也实现了ToString
}