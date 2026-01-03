fn main() {

}

struct Point {
    x: i32,
    y: i32,
}

fn process_tuple((a, b, c): (i32, f64, &str)) {
    println!("解构元组：a={a}, b={b}, c={c}");
}

fn process_array([first, mid@.., last]: [i32;5]) {
    println!("解构数组: [{first} {mid:?} {last}]");
}

fn process_struct(Point {x:a, y:b}: Point) {
    println!("解构Struct: a={a}, b={b}");
}

fn process_struct_with_bind(point @ Point {x, y}: Point) {
    println!("解构Struct: x={x}, y={y}");
}

