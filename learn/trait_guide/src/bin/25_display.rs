use std::fmt::{self, Display, Formatter};

struct Point { x: i32, y: i32 }

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{p}");    //  可直接使用 {} 来输出
}