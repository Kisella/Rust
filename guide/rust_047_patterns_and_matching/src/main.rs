fn main() {
    // let 语句是模式
    let a = 5;
    let (x, y, z) = (1, 2, 3);
    // let (q, w) = (1, 2, 3);  // Compiling error, 模式不匹配
    println!("{}", x);

    // for循环中，模式是紧随for关键字后的值
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("v[{}] = {}", index, value);
    }

    // 函数参数也可以是模式d
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {}) ", x, y);
}
