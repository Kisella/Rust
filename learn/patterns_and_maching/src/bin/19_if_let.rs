fn main() {
    let op = Option::from(10);

    if let Some(value) = op {
        println!("Value : {value}");
    }

    // 效果等价与
    match op {
        Some(value) => println!("Value: {value}"),
        _ => (),
    }

    let op = Option::from(10);

    let Some(value) = op else {
        panic!("No value!");
    };

    println!("x = {value}");

    let mut stack = vec![1, 2, 3];

     while let Some(top) = stack.pop() {
        println!("Poped {top}");
     }

     let map = vec![("a", 1), ("b", 2)];

     for (k, v) in map {
        println!("{k}: {v}");
     }
}