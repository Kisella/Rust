use case_019::{Summary, Tweet};
fn main() {
    println!("case 35 & 36");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of couser, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: \"{}\"", tweet.summarize());
}
