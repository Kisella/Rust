/*
 * 可以使用模式来解构struct、enum、tuple, 从而引用这些类型值的不同部分
 */

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; //  使用模式对结构体进行结构，获得结构体的子段值
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // 由 let Point { x: x, y: y } = p; 简写而来，这里的x和y是新创建的两个量
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 结构体使用match匹配
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),    //  字段y为0，字段x随意
        Point { x: 0, y } => println!("On the y axis at {}", y),    //  字段x为0，字段y随意
        Point { x, y } => println!("On neither axis: ({}, {})", x, y), //  匹配剩余情况，打印值
    }
}
