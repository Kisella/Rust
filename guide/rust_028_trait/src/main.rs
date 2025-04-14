use rust_028_trait::Summary;
use rust_028_trait::Tweet;

fn main() {
    let tweet = Tweet {
        username:String::from("horse_ebooks"),
        content:String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet:\n{}", tweet.summarize());
}
