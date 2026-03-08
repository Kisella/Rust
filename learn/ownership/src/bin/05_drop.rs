#[derive(Debug, Default)]
struct Val {
    x: i32,
    y: f64,
}

impl Drop for Val {
    fn drop(&mut self) {
        println!("Dropping Val: {:?}", self);
    }
}

fn main() {
    let v1 = Val::default();
    let v2 = Val { x: 20, y: 2.71 };
    let v3 = Val { x: 30, y: 1.61 };
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    // 显式调用drop函数
    drop(v1); // v1被丢弃，调用drop方法
    println!("After dropping v1");
    {
        let v = v2; // v2被移动到v中，v2不再可用
    }   //  v生命周期结束，调用drop方法
    // v3会在main函数结束时自动被丢弃，调用drop方法
}