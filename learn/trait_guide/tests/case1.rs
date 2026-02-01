// trait定义
mod definition_of_trait {
    // 自定义类型NewsArticle
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        _content: String,
    }

    trait Summary {
        fn summarize(&self) -> String; 
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    #[test]
    fn test() {
        let news = NewsArticle {headline: "Never Pro Miss".to_string(), location: "BeiHai".to_string(), author:"Ayaban".to_string(), _content: "Hello world!".to_string()};
        println!("{}", news.summarize());
    }
}

// trait的默认实现
mod default_implementation {
    trait HelloWorld {
       fn print_hello_world(&self) {
        println!("Hello world!");
       } 
    }

    impl HelloWorld for i32 {}
    impl HelloWorld for &str {
        fn print_hello_world(&self) {
            println!("Hello world! {self}");
        }
    }
    #[test]
    fn test() {
        520.print_hello_world();
        "520".print_hello_world();
    }
}