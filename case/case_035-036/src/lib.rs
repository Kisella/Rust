/*
 * trait的定义
 */ 
/*
 * case 35 
 * 使用trait关键字定义一个特性
 * trait里的方法可以只写函数的签名，也可以写完整的函数实现
 */  
pub trait Summary {
    fn summarize(&self) -> String;  //  定义了一个函数签名，以分号结束
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*
 * case 36
 * 为某个类型实现trait
 * 使用“impl trait名 for 类型名”的形式为类型实现特性
 * 为类型实现trait时，必须将trait里定义的还没有实现的方法都实现了
 */  
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
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
}

