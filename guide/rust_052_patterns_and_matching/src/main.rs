struct Point {
    x: i32,
    y: i32,
}

// 使用下划线(_)来忽略整个值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
fn main() {
    // 解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("(feet: {}, inches: {})", feet, inches);
    println!("Point {{x: {}, y: {} }} ", x, y);

    // 使用下划线(_)来忽略整个值
    foo(3, 4);

    // 使用模块中的下划线(_)来忽略整个值的一部分
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // 使用_开头命名来忽略未使用的变量
    let _x = 5;
    let y = 10; //    warning

    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {   //  Compiling error, 使用_s会发生绑定动作，所有权会移动
    if let Some(_) = s {     //  使用_不会发生绑定，所有权不移动
        println!("Found a string");
    }
    println!("{:?}", s);
}
