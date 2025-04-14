/*
 * FnOnce
 *     如果一个闭包将捕获的值移出主体(将值返回或传递给其他函数)，那么它只能实现FnOnce。因为一旦值被移出，闭包就不能再次调用
 *
 * FnMut
 *     适用于不会移出值，但可能会修改捕获的值的闭包，这类闭包可以被多次调用
 * Fn
 *     适用于既不移出值，也不修改捕获值得闭包，也适用于完全不捕获环境中的值的闭包。这类闭包可以被多次调用，特别适用于需要并发调用闭包的场景
 */

enum Option<T> {
    Some(T),
    None,
}
use crate::Option::{None, Some};
/*
 * 方法unwrap_or_else要求参数f实现了FnOnce的特性，故参数f是一个闭包，且只能调用一次
 * 因为参数闭包f的约束仅为实现了FnOnce，故所有的闭包都可以作为unwrap_or_else方法的参数，因为只要是个闭包都实现了FnOnce
 */
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width); // sort_by_key方法的参数闭包需要能被调用多次，故要求实现FnMut特性
    println!("{list:#?}");

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    /*
     * 闭包参数在环境中捕获了sort_operations和value两个值，且在捕获了value之后又将其移出了闭包体(value所有权最终给了sort_operations)，故闭包参数只能实现FnOnce(只能调用一次)，不符合FnMut的特性约束。
     */
    list.sort_by_key(|r| {
        // sort_operations.push(value);    //  Compiling error, 捕获的变量发生所有权移动, 不符合FnMut特性约束
        sort_operations.push(value.clone()); //  对捕获的变量克隆，不发生移动，符合FnMut特性约束
        r.width
    });
    println!("{list:#?}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // 在闭包里修改外部可变的变量，默认捕获的是被修改变量的可变引用，符合FnMut特性约束
        r.width
    });
    println!("{list:#?}");
    println!("{num_sort_operations}");
}
