fn main() {
    let arr = [10, 20, 30, 40];
    
    match arr {
        [first, _, third, _] => println!("first = {first}, third = {third}")
    }

    match arr {
        [first, .., last] => println!("range: {first} _ {last}"),
    }

    match arr {
        [0..=20, second, ..] => println!("start with 10, second = {second}"),
        _ => ()
    }

    let vector = vec![10, 20 ,30 ,40];
    // - 对于不定长的数组，需要使用切片进行匹配
    match &vector[..] {
        [] => println!("Empty vector"),
        [x] => println!("Single element: {x}"),
        [first, second] => println!("Two element: {first} {second}"),
        [first, .., last] => println!("First: {first}, Last: {last}")
    }
}