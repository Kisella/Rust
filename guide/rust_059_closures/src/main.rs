#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list1 = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut list2 = list1.clone();

    let mut num_sort_operations1 = 0;
    list1.sort_by_key(|r| {
        num_sort_operations1 += 1; // 在闭包里修改外部可变的变量，默认捕获的是被修改变量的可变引用，符合FnMut特性约束
        r.width
    });
    println!("{list1:#?}");
    println!("{num_sort_operations1}");

    let mut num_sort_operations2 = 0;
    list2.sort_by_key(move |r| {    //  强制闭包获得捕获变量的所有权
        //  使用
        num_sort_operations2 += 1; // 为了满足FnMut的特性约束，内部num_sort_operations2获得的是外部num_sort_operations2的副本(copy), 故对内部um_sort_operations2值的修改不会改变外部的值
        r.width
    });
    println!("{list2:#?}");
    println!("{num_sort_operations2}");     //  打印的是外部的num_sort_operations2，其结果为0不变

}

// F: FnOnce(&mut T)  报错, 闭包在函数中需要被调用多次
// F: Fn(&mut T)      告警, 参数没必要定义为可变的
// F: FnMut(&mut T)   适合
fn for_each_mut<T,F: FnMut(&mut T)> (v: &mut Vec<T>, mut f:F) {
    for x in v.iter_mut() {
        f(x);   
    }
}

pub struct Analyzer<F> {
    postprocess: F
}
impl<F> Analyzer<F> {   //  为所有的F实现方法
    fn process(&self, n:i32) -> i32 {/* .. */ 0 }
}
impl<F: Fn(i32) -> i32> Analyzer<F> {   //  为实现了Fn特性的F实现方法
    pub fn pipeline(& self, n: i32) -> i32 {    //  Fn特性不影响值，传入不可变引用
        let n = self.process(n);
        (self.postprocess)(n)
    }
}
impl<F: FnMut(i32) -> i32> Analyzer<F> {        
    pub fn pipeline1(&mut self, n: i32) -> i32 {   //   FnMut可改变值，传入可变引用
        let n = self.process(n);
        (self.postprocess)(n)
    }
}
impl<F: FnOnce(i32) -> i32> Analyzer<F> {        
    pub fn pipeline3(self, n: i32) -> i32 {     //  FnOnce特征只调用一次，需要移出所有权，传入所有权
        let n = self.process(n);
        (self.postprocess)(n)
    }
}

