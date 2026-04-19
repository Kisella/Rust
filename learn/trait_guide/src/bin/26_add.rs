use std::ops::Add;
struct Point {
    x: f64,
    y: f64,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;   //  实现了Add特征的类型可以进行加法运算
    println!("p3: ({}, {})", p3.x, p3.y);
}
