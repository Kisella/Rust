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

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {}

/*
 * case 45 truth 
 * 返回实现Trait的类型
 * "impl Trait"作为返回类型是为了能够抽象具体类型，只暴露trait的接口，这样可以在不暴露具体类型的情况下提供灵活性。
 */ 
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}
fn returns_summarizable_1() -> Tweet {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}

/*
 * `impl Trait` 作为返回类型的主要目的是抽象返回的具体类型，只暴露 trait 的接口，这样可以在不暴露具体类型的情况下提供灵活性。例如，未来如果需要更改返回的具体类型，只要新类型也实现了相同的 trait，就不需要修改函数的签名，调用方的代码也不受影响。而直接返回具体类型则会将实现细节暴露给调用者，一旦需要更改返回类型，所有调用该函数的地方都需要调整。
 * `returns_summarizable` 返回 `impl Summary`，而 `returns_summarizable_1` 返回 `Tweet`。虽然现在它们返回的是相同的 `Tweet` 结构体，但 `impl Summary` 的版本隐藏了具体的返回类型，只保证返回的对象实现了 `Summary` trait。这样做的好处是，如果未来需要更改内部实现，比如返回另一个实现了 `Summary` 的类型，只需修改函数内部的实现，而无需改变函数签名，调用方的代码也不需要任何修改。而返回具体类型的版本则不具备这种灵活性，一旦更改返回类型，所有依赖该函数的地方都需要调整。
 */ 

/*
 * case 45 faker
 * "impl Trait"作为返回值要求必须只能返回单一类型
 * impl Trait 的“单一类型”限制迫使开发者提前设计清晰的接口契约，避免滥用动态类型导致运行时错误。
 */ 
// fn returns_summarizable2(switch: bool ) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                 hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false
//         }
//     }
// }


