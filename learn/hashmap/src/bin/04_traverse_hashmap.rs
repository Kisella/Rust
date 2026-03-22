fn main() {
    use std::collections::HashMap;

    let vec = vec![(1, "value1".to_string()), (2, "value2".to_string())];
    let hashmap = vec.into_iter().collect::<HashMap<_,_>>();
    for (key, value) in &hashmap {
        println!("{}: {:?}", key, value);
    }
    println!("{hashmap:?}");
}