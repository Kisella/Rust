// #[derive(Default)] 提供快速实现
#[derive(Default)]
struct Config {
    debug: bool,
    retries: u32,
}

fn main() {
    let c1 = Config::default();

    // 与结构体更新语法搭配
    let c2 = Config {
        debug: true,
        ..Default::default()  // 使用 ..Default::default() 来填充剩余字段
    };
}