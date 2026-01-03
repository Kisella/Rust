fn main() {
    // let s = String::from("hello");

    // match s {
    //     mut x => x.push_str(" world"),
    // }

    // println!("{s}");  //  拒绝，所有权已移动给x

    let mut s = String::from("hello");

    match &mut s {
        x => x.push_str(" world"),  // 此处的 x 为 &mut String 类型, s 的可变借用
    }

    println!("{s}");  //  接受，所有权未发生移动

    let x = 10;
    let ref1 = &x;
    let ref ref2 = x;  //   两种写法完全等价
    assert_eq!(ref1, ref2);

    let mut s = String::from("hello");

    match s {
        ref mut x => x.push_str(" world"),  // 此处的 x 为 &mut String 类型, s 的可变借用
    }

    println!("{s}");  //  接受，所有权未发生移动


    match (&(10, 20), &(30, 40)) {
        ((x,y), &(a,b)) => println!("{x} {y} {a} {b} "),    //  x/y为&i32, a/b为i32
    }
}