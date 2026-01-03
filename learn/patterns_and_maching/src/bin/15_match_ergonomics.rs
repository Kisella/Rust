fn main() {
    let m = Some(String::from("hellow"));


    // 本源写法，需要手动拆解和添加内层ref
    match &m {
        &Some(ref msg) => println!("val = {msg}"),
        None => println!("none"),
    }

    // 人体工学写法，自动将外部的引用带入内部
    match &m {
        Some(msg) => println!("val = {msg}"),
        None => println!("none"),
    }

    let x = Some(10);
    
    match &x {
        &Some(val) => println!("val = {val}"),  //  未触发人体工学，值发生复制，编程规范不推荐
        None => println!("none")
    }

    match &x {
        &Some(ref val) => println!("val = {val}"),  //  未触发人体工学，拿到内部数据的引用，可选
        None => println!("none")
    }

    match &x {
        Some(val) => println!("val = {val}"),  //  触发人体工学，内部数据自动代入引用，可选
        None => println!("none")
    }

    match (&(10, 20), &(30, 40)) {
        ((x,y), &(a,b)) => println!("{x} {y} {a} {b} "),    //  x, y: &i32; a, b: i32
    }

    match &&&&&&&x {
        Some(val) => println!("val = {val}"),   //  x: &i32
        None => println!("none")
    }

    match &mut(10, &(20, 30)) {
        // x: &mut i32, y: &i32, z: &i32
        (x, (y, z)) => println!("values: {x} {y} {z}")
    }

    match &(10, &mut (20, 30)) {
        // x: &mut i32, y: &i32, z: &i32
        (x, (y, z)) => println!("values: {x} {y} {z}")
    }

    match &(&10, 20) {
        (x, y) => println!("values : {x} {y}"),
    }


}