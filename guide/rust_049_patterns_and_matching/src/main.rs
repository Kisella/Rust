fn main() {
    // 模式可以直接匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("over three"),
    }

    // 命名的变量是可匹配任何值的无可辩驳模式
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Match, y = {}", y),    //  此y是一个新变量，作用域在match块里
        // Some(z) if z == y  => println!("Match, y = {}", z),  //  若需要使用外层的y，里层就不应该使用y作为匹配值
        _ => println!("Default case, x = {:?}", x),
    }
    println!("At the end: x = {:?}, y = {}", x, y); //  match表达式结束，里面新的y变量失效，外层定义的y恢复使用

    // 在match表达式里可以使用管道符(|), 匹配多种模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 使用 ..= 来匹配某个范围的值
    let x = 5;
    match x {
        1..=5 => println!("one through five"),  //  匹配 1 2 3 4 5
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
