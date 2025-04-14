pub trait Summary {
    fn summarize(&self) -> String {     //  默认实现
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})",self.headline, self.author, self.location)
    // }

    // summarize在Summary这个trait已经有默认的实现了，NewsArticle就可以直接采用默认实现而不进行自己的实现
}

pub trait Summary1 {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {     
        format!("(Read more from {}...)", self.summarize_author())
    }
    // 默认实现的方法可以调用trait中其他的方法，即使这些方法没有默认实现
}

impl Summary1 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

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
    // 默认实现的重新实现
}
