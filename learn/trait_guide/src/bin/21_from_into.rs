#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 为 Point 实现 From trait，使其能够从一个(i32, i32)的元组转换而来
impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

impl From<Point> for (i32, i32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

fn main() {
    // 使用 From trait 进行转换
    let point: Point = Point::from((5, 10));
    println!("{point:?}");

    // 使用 Into trait 进行转换
    let another_point: Point = (15, 20).into();
    println!("{another_point:?}");

    // 将 Point 转换回元组
    let tuple: (i32, i32) = point.into();
    println!("{tuple:?}");

    let another_tuple: (i32, i32) = <(i32, i32)>::from(another_point);
    println!("{another_tuple:?}");
}