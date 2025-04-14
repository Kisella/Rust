/*
 * 使用..来忽略值的剩余部分
 * 使用 match 守卫来提供额外的条件
 */
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let number = (2, 4, 8, 16, 32);
    match number {
        (first, .., last) => {
            //  使用..只取头尾值
            println!("Some numbers: {}, {}", first, last);
        }
    }
    // match number {
    //     (.., seconde, ..) => {      //  Compiling error, 只能使用一次..否则会有歧义
    //         println!("Some numbers: {}", seconde);
    //     }
    // }

    // match守卫就是match arm模式后额外的if条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
