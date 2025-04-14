/*
 * 为泛型实现特性约束
 */ 
use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

/*
 * case 39
 * Trait作为参数，准确来说是有Trait实现作为约束的泛型作为参数
 * 只有一个Trait实现作为约束时，可以使用“impl Trait名称”的简便写法
 */
pub fn notify0(item: &impl Summary) {
    //  impl Trait 写法
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}

/*
 * case 40
 * 以实现某些Trait作为约束条件，称为泛型参数的Trait Bound (特性约束)
 * impl Trait是特性约束的一种简单情况的简写
 * 特性约束的写法为在函数声明的泛型参数后加冒号接要求实现的Trait名称
 */

pub fn notify2<T: Summary>(item: &T) {
    //  Trait Bound 写法
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

/*
 * case 41
 * Trait Bound (特性约束)可以使用where关键字以应该更复杂的情况
 */
pub fn notify4<T>(item: &T)
//    where 写法
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

pub fn notify5<T>(item1: &T, item2: &T)
where
    T: Summary,
{
}

/*
 * case 42
 * 使用加号(+)来指定多个Trait Bound
 */
pub fn notify6(item: &(impl Summary + Display)) {}
pub fn notify7<T: Summary + Display>(item: &T) {}
pub fn notify8<T>(item: &T)
where
    T: Summary + Display,
{
}

/*
 * case 43
 * Trait Bound (特性约束)可以使用where关键字以应该更复杂的情况
 */
fn notify9<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}
fn notify10<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{0}
