fn main() {
    // 通过泛型定义一个复合类型族
    struct Point<T> {
        x: T,
        y: T,
    }

    // 自动推断为 Point<i32> 类型
    let p = Point {
        x: 0,
        y: 0,
    };

    // 显式指定为 Point<i32> 类型
    let p: Point<i32> = Point {
        x: 0,
        y: 0,
    };

    //  turbofish 指定为 Point<f64> 类型
    let f = Point::<f64> {
        x: 3.14,
        y: 3.14,
    };

    type Pi32 = Point<i32>;
    let p: Pi32 = Point { x: 3, y: 4 };
    type ResultI32<E> = Result<i32, E>;
}