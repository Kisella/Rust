use std::fmt::Display;

struct MyString(String);    //  NewType, 包裹外部类型

impl Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyString says: {}", self.0)
    }
}

fn main() {
    let s = MyString(String::from("hello"));
    println!("{s}");
}