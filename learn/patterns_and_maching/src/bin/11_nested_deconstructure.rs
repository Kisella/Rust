fn main() {
    let arr = [(10, 20), (20, 30), (30, 40)];

    match arr {
        [(a, b), _, (30, c)] => println!("{a} {b} {c}"),
        _ => println!("default"),
    }

    let arr = [(1 ,2, 3), (10 ,20, 30), (100 ,200, 300), (10 ,20, 30), (1 ,2, 3)];

    match arr {
        [(a,b, c), second, (d, ..), .., last] => println!("do nothing"),
        _ => println!("default"),
    }
}