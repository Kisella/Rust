/*
 * trait的默认实现
 */ 
/*
 * case 37
 * trait的默认实现
 * 在trait的定义中存在函数体的方法被称为“默认实现”
 * 当在特定类型实现trait时，我们可以保留或者覆盖那个默认实现
 */  
pub trait Summary {
    fn summarize(&self) -> String {
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

// 为Tweet实现Summary特性，大括号里为空，表示采用Summary的默认方法
impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

