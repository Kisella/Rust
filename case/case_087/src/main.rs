use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Node {
    value : i32,
    str: String,
}

fn main() {
    /*
     * case 87
     * 使用as_ref来读取或修改Option<T>中的值
     * 使用as_ref可避免Option<T>的所有权移动，是比克隆更推荐的做法
     */ 

    let a = Some(Rc::new(5));
    let b = a.as_ref().unwrap();
    println!("{a:?} {b}");
    let a = Some(Rc::new(RefCell::new(Node {value:555, str:String::from("Hello World!")})));
    (*a.as_ref().unwrap().borrow_mut()).value = 777;    
    println!("{a:?}");
    (*(&a).as_ref().unwrap().borrow_mut()).value = 666;     //  这里对a和对&a进行操作效果相同
    println!("{a:?}");
}