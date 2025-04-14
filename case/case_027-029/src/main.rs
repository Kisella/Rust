/*
 * 泛型的声明
 */ 
/*
 * case 27
 * 在函数定义中使用泛型
 * 函数使用泛型参数前需要进行声明，泛型参数的声明放在尖括号(<>)里，并把尖括号放在函数名与参数列表之间
 * 没有任何限制的泛型参数T是无法比较大小的，T需要先实现PartialOrd特性
 * 修复请看case 44
 */
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            //  没有任何限制的泛型参数T是无法比较大小的
            largest = item;
        }
    }
    largest
}

/*
 * case 28
 * 在结构体定义中使用泛型
 * 结构体使用泛型参数前需要进行声明，泛型参数的声明放在尖括号(<>)里，并把尖括号放在结构体名称后
 * 可以为结构体声明多个的泛型参数，使它的字段能够存放不同类型的数据
 */
struct Point<T> {
    x: T,
    y: T,
}
struct Point1<T,U> {
    x: T,
    y: U,
}

/*
 * case 29
 * 在枚举中定义中使用泛型
 * 枚举使用泛型参数前需要进行声明，泛型参数的声明放在尖括号(<>)里，并把尖括号放在枚举名称后
 * 可以为枚举声明多个的泛型参数，使它的变体能够存放不同类型的数据
 */
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let float_and_integer = Point { x: 5, y: 6.5 }; // Point要求两个字段的类型是一致的
    println!("Compiling error!");

    let integer = Point1 { x: 7, y: 3 };
    let float_and_integer = Point1 { x: 5, y: 6.5 };
}
