fn main() {
    let x = 10;
    let ref r = x;

    // 需要手动解引用
    match r {
        val => i32::abs(*val),
    };

    // 模式中使用&T, 自动拆掉引用
    match r {
        &val => i32::abs(val),
    };

    // 可连写多层&, 拆掉多层借用
    match &&r {
        &&&val => i32::abs(val),
    };

    // let s = String::from("hello");
    // let r = &s;

    // match r {
    //     &val => println!("{val}"), // 拒绝，不能依靠&拆解得到堆上的数据
    // }
}