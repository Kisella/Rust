#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoiner with data `{}`!", self.data);
    }
}

/*
 * case 78
 * 使用std::mem::drop强制丢弃值
 * Rust不允许手动调用Drop特性中的drop方法
 * 如需将一个值在作用域结束前提前丢弃，可使用标准库中的std::mem::drop方法
 * std::mem::drop的调用不会干扰Rust的自动清理机制，不会产生重复释放问题
 */ 
fn main() {
    println!("case 78");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // d.drop();      //   Rust不允许手动调用drop方法
    println!("Compiling error!");
    drop(c);        
    //   标准库drop函数将取得c的所有权，且函数体为空，这样就让c走出了作用域，进而调用drop方法进行清理
    // println!("{c:?}");     //  c已被提前清理


    /*
    * case 79
    * 使用厕所闭包也能实现值的清理。
    */ 
    println!("\ncase 79");
    println!("{d:?}");  
    let toilet = |_|();     //  写作|_|{}效果相同
    // 厕所闭包与std::mem::drop类型，也是获取值的引用后什么都不做，从而实现值的清理
    toilet(d);
    println!("the value d has got into toilet.");
}
    
// main函数结束，没有值需要清理