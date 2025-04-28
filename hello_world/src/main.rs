use std::collections::HashMap;

#[derive(Debug)]
struct Foo {
    a: String,
    b: usize,
}

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert(1, Foo{a:"hello".to_string(), b:32});
    hashmap.insert(2, Foo { a: "world".to_string(), b: 64 });
    println!("{hashmap:#?}");


    hashmap.get_mut(&1).unwrap().a = "I love you".to_string();
    println!("{hashmap:#?}");
}
