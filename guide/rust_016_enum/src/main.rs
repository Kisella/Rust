// 枚举: 允许我们列举所有可能的值来定义一个类型
// 定义枚举: 使用enum关键字来定义枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
// rust中允许将数据附加到枚举的变体中
// 优点: 1.不需要额外使用struct    2.每个变体可以拥有不同的类型以及关联的数据量

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Wring(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}

    fn new_move(x: i32, y: i32) -> Message {
        Message::Move { x, y }
    }
}
// 可以使用impl关键字为枚举定义方法和关联函数

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // 使用双冒号(::)来获取枚举值
    println!("{:#?} {:#?}", four, six);
    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6("::1".to_string());
    println!("{:?}", home);
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("{}.{}.{}.{}", a, b, c, d);
        }
        _ => () //  使用_表示剩下的未匹配的情况
    }
    // 使用match和模式匹配来访问枚举中的元组变体中的元素

    println!("{:?}", loopback);

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Wring(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    q.call();
    let m1 = Message::new_move(24, 48);
    println!("{:#?}", m1);
}

fn route(ip_kind: IpAddrKind) {}
