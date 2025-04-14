use rust_029_trait::NewsArticle;
use rust_029_trait::Summary;
fn main() {
    let article = NewsArticle {
        headline:String::from("Penguins win the Stanley Cup Championship!"),
        content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author:String::from("Iceburgh"),
        location:String::from("Pittsburgh, PA, USA"),
    };
    println!("1 new newsarticle:{}", article.summarize())
}
