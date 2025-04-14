/*
 * trait定义里没有实现的函数签名可以被其他有默认实现的方法调用
 */ 
use case_038::{NewsArticle, Summary, Tweet};
fn main() {
    println!("case 38");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    // 为NewsArticle实现Summary特性时已重写summarize方法，summarize的默认实现被覆盖
    println!("New article available! \"{}\"", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of couser, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // 为Tweet实现Summary特性时代码块为空，summarize方法使用默认实现
    println!("1 new tweet: \"{}\"", tweet.summarize());
}
