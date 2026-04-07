struct Type1;
trait Demo {
    fn demo(&self);
    fn func(&self, other: Self) -> Self
    where Self: Sized;  //  ✅ 添加 Trait 约束，显式标记为不可调用
    fn func2(num: i32)
    where Self: Sized;  //  ✅ 添加 Trait 约束，显式标记为不可调用 
}

impl Demo for Type1 {
    fn demo(&self) {
        println!("Tpye1 demo");
    }

    fn func(&self, other: Self) -> Self
        where Self: Sized {
        println!("Type1 func");
        other
    }

    fn func2(num: i32)
        where Self: Sized {
        println!("Type1 func2 with num: {}", num);
    }
}

fn process(d: &dyn Demo) {
    d.demo();           // ✅ 可以调用
    // d.func(Type1);  // ❌ 不可调用
    // d.func2(42);    // ❌ 不可调用
}

fn main() {
    let t1 = Type1;
    process(&t1);
}