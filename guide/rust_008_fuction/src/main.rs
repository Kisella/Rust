fn main() {
    another_function_one();
    // rust中不关心函数定义在哪，只要在同一个文件中存在函数的定义就能进行调用

    let x = 67;
    let y = 25;
    another_function_two(x, y);

    let y = {
        let x = 1;
        x + 3 //  x + 3 (没有带分号)也是一个表达式，它就是整个块表达式的值
    };
    // 使用大括号可以构建一个块表达式
    println!("The value of y is {:?}", y);

    let y = {
        let x = 1;
        x + 3; //  若 x + 3 带上分号，就变成了一个语句，语句的返回值是一个空的tuple, 块表达式的值也就变成了一个空的tuple
    };
    // 使用大括号可以构建一个块表达式
    println!("The value of y is {:?}", y);

    println!("{}", five());
    println!("6 + 5 = {}", plus_five(6));
}

fn another_function_one() {
    println!("Another fuction one");
}

// 在函数的声明里，必须显式声明每个参数的类型
fn another_function_two(x: u32, y: u32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}
fn plus_five(x: i32) -> i32 {
    x + 5
}
// 在 -> 符号后声明函数返回值的类型，函数的返回值不能命名
// 在Rust里，函数的返回值一般就是函数体中最后一个表达式的值
// 若想提前返回可使用return关键字，并指定一个值