pub trait Summary {
    fn summarize(&self) -> String;
    // fn summarize1(&self) -> String;
}
// 使用trait关键字定义trait
// 一个trait可以有多个方法体，每个方法签名占一行，以分号(;)结尾
// 实现该trait的类型必须提供具体的方法实现

pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",self.headline, self.author, self.location)
    }
}
// 使用impl关键字和for关键字为类型实现trait
// 在impl的块里需要对trait里的方法签名进行具体的实现

pub struct Tweet {
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}