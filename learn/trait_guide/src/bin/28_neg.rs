use std::ops::Neg;
struct Point {
    x: f64,
    y: f64,
}
impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = -p1;   //  实现了Neg特征的类型可以进行取反运算
    println!("p2: ({}, {})", p2.x, p2.y);
}