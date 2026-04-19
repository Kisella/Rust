use std::ops::AddAssign;
struct Point {
    x: f64,
    y: f64,
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let mut p2 = Point { x: 3.0, y: 4.0 };
    p2 += p1;   
    println!("p2: ({}, {})", p2.x, p2.y);
}