use rust_030_trait::{notify, notify1, NewsArticle, Summary};
fn main() {
    let article = NewsArticle {
        headline:String::from("Penguins win the Stanley Cup Championship!"),
        content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author:String::from("Iceburgh"),
        location:String::from("Pittsburgh, PA, USA"),
    };
    notify(&article);
    notify1(&article);
}
