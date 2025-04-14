#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 针对Point<T>来实现方法时，需要在impl关键字后加上<T>, 表明是针对泛型T(而不是具体的某个类型)来实现方法

impl Point<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}
// 当然，也可以只针对具体的Point类型来实现方法。针对具体的类型来实现方法的时候，impl关键字后不需要加上<T>。上面的例子中，x1这个方法只在Point<i32>这个具体的类型上才有，而其他Point<T>的类型没有x1这个方法

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    OK(T),
    Ert(E),
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
// struct的泛型类型参数可以和方法的泛型类型参数不同

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let integer = Point{x:5, y:1.0};     //  Compilying error, 实例化Point时，x和y的类型得是一致的
    println!("{:#?}\n{:#?}", integer, float);

    let integer_ = Point1 { x: 5, y: 1.5 }; //  可使用多个类型参数存储不同的类型
    println!("{:#?}", integer_);

    integer.x();
    integer.x1();
    float.x();
    // float.x1();      //  Compilying error, Point<f64>这个类型没有x1这个方法

    let p1 = Point2{x:5, y:4};
    let p2 = Point2{x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
