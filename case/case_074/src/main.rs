/*
 * case 74
 * 闭包作为参数传递时需要借助泛型参数和Trait
 * 闭包参数存在三种类型：
 * FnOnce，闭包获取了被捕获值的所有权，只能被调用一次
 * FnMut， 闭包获取了被捕获值或被传入值的可变引用，可以被调用多次
 * Fn，闭包获取了被捕获值或被传入值的不可变引用，可以被调用多次
 */

//  f作为一个闭包参数被传入，它的类型是FnMut，可以被多次调用，且可以改变被传入的值
fn for_each_mut<T, F: FnMut(&mut T)>(v: &mut Vec<T>, mut f: F) {
    for x in v.iter_mut() {
        f(x);
    }
}

pub struct Analyzer<F> {
    postprocess: F,
}
impl<F> Analyzer<F> {                   //  为所有的Analyzer<F>结构体实现方法
    fn process(&self, n:i32) -> i32 {n}
}
impl<F: Fn(i32) -> i32> Analyzer<F> {   //  为实现了Fn闭包特性的Analyzer结构体实现方法
    pub fn pipeline1(&self, n:i32) -> i32 {    //  &self, 代表取得捕获变量的不可变引用
        let n = self.process(n);
        (self.postprocess)(n)
    }
}
impl<F: FnMut(i32) -> i32> Analyzer<F> {   //  为实现了FnMut闭包特性的Analyzer结构体实现方法
    pub fn pipeline2(&mut self, n:i32) -> i32 {     //  &mut self, 代表取得捕获变量的可变引用
        let n = self.process(n);
        (self.postprocess)(n)
    }
}

impl<F: FnOnce(i32) -> i32> Analyzer<F> {    //  为实现了FnMut闭包特性的Analyzer结构体实现方法
    pub fn pipeline3(self, n:i32) -> i32 {   //  self, 代表取得捕获变量的所有权
        let n = self.process(n);
        (self.postprocess)(n)
    }
}

fn main() {
    println!("Hello, world!");
}
