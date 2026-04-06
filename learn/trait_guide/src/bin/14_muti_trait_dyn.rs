// 定义基础TraitTrait
use std::fmt::Debug;

trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

// 定义类型，并实现基础trait
#[derive(Debug)]
struct S;
impl Foo for S {
    fn foo(&self) {
        println!("foo");
    }
}

impl Bar for S {
    fn bar(&self) {
        println!("bar");
    }
}

// 定义组合trait，依赖基础trait
trait FooBarDebug: Foo + Bar + Debug {}

// 使用Blanket实现，为所有实现了Foo、Bar和Debug的类型自动实现FooBarDebug
impl<T> FooBarDebug for T where T: Foo + Bar + Debug {}

// 定义函数，接受一个组合trait对象作为参数
fn call_all(x: &dyn FooBarDebug) {
    x.foo();
    x.bar();
    println!("{:?}", x);
}

fn main() {
    let s: S = S;
    call_all(&s);
}