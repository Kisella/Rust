use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        //  默认实现
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})",self.headline, self.author, self.location)
    // }

    // summarize在Summary这个trait已经有默认的实现了，NewsArticle就可以直接采用默认实现而不进行自己的实现
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // 默认实现的重新实现
}

// 将Trait作为参数
// impl Trait语法：适用于简单情况
pub fn notify(item: &impl Summary) {
    //   表明item是实现了Summary这个trait的某个类型的实例
    println!("Breaking news! {}", item.summarize());
}

// Trait bound语法，可用于复杂情况
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    //   表明item是实现了Summary这个trait的某个类型的实例
    println!("Breaking news! {}", item1.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// 使用加号(+)指定多个Trait bound
pub fn notify4(item: &(impl Summary + Display)) {
    //   表明item是实现了Summary这个trait的某个类型的实例
    println!("Breaking news! {}", item.summarize());
}

pub fn notify5<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound使用where子句来指定trait的约束
pub fn notify6<T: Summary + Display, U: Clone + Debug>(a: &T, b: &U) -> String {
    format!("Breaking news! {}", a.summarize())
}
// ==>
pub fn notify7<T, U>(a: &T, b: &U) -> String
where                       //  在方法签名后指定where子句，指定trait的约束
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

// Trait作为返回类型
pub fn notify8() -> impl Summary {
    NewsArticle {
        headline:String::from("Penguins win the Stanley Cup Championship!"),
        content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author:String::from("Iceburgh"),
        location:String::from("Pittsburgh, PA, USA"),
    }
}
// impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错