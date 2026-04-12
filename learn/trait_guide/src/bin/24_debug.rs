use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{p:?}");
    println!("{p:#?}");

    let c = Config::default();
    println!("{c:?}");
}

#[derive(Default)]
struct Config {
    debug: bool,
    retries: u32,
}

// 手动实现Debug, 定制输出格式
impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "debug is [{}], retries is [{}]", self.debug, self.retries)
    }
}