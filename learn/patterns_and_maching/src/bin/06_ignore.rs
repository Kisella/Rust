fn main() {
    struct Point {x:i32, y:i32, z:i32}

    let p = Point {x:1, y:2, z:3};
    match p {
        Point {x, ..} => println!("x: {x}"),
    }
}