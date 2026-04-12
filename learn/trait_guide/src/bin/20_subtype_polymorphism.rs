trait Shape {
    fn area(&self) -> f64;
}
struct Circle { r: f64, }
struct Square { a: f64, }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.a * self.a }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { r: 5.0 };
    let square = Square { a: 4.0 };
    print_area(&circle);
    print_area(&square);
}