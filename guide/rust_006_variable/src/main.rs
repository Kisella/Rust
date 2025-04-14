fn main() {
    let x = b'A';
    println!("{}", x);

    let x = 6.5f32;
    println!("{}", x);

    let x = 358_655u64;
    println!("{}", x);
    // 除了byte类型外，所有的数值字面值都运行使用类型后缀

    let x = 98_222;
    println!("{}", x);
    // 整数类型可通过添加下划线增加可读性

    let x = 0xff;
    println!("{}", x);

    let x = 0o77;
    println!("{}", x);

    let x = 0b1111_0000;
    println!("{}", x);

    let x = true;
    println!("{}", x);

    let x = 'A';
    println!("{}", x);

    let x = '😂';
    println!("{}", x);
}
