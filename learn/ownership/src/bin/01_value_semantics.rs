
fn main() {
    #[derive(Clone, Copy)]
    struct Val {
        x: i32,
        y: f64,
    }

    let v1 = Val{x:1, y:3.14};
    let v2 = v1;
    println!("{}, {}", v1.x, v1.y);     // 拒绝，已发生移动
    println!("{}, {}", v2.x, v2.y);
}