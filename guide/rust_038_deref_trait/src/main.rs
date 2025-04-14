/*
 * 函数和方法的隐式解引用转化(Deref Coercion)
 * 定义: 若T实现了Deref特征，Deref Coercion可以把T的引用转换为T经过Deref操作后生成的引用
 * 应用场景: 当某类型的引用传递给函数或方法，但它的类型与定义的参数类型不匹配时，Deref Coercion就会自动发生，编译器会对deref方法进行一系列的调用，来把它转化为所需的类型。这一过程在编译过程中完成，没有额外性能开销
 */

use std::ops::Deref;

fn hello(name:&str) {
    println!("Hello, {}", name);    
}
fn main() {
    let m = MyBox::new(String::from("Rust"));
    let i1 = m.deref();
    let i2 = (*i1).deref();
    hello(&m);
    hello(&(*m)[..])    //  MyBox类型没有实现Deref特征时的调用方法
    // &m : &MyBox<String>
    // &m => m.deref() => &String => &str      String类型也实现了Deref特征
    
    // 只要类型实现了Deref特征，rust就会自动分析类型并不断尝试调用类型的deref方法，来让类型参数与函数或方法定义的参数类型匹配。而且这一过程是在编译过程中完成的，所有在运行时不会产生额外的开销
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
