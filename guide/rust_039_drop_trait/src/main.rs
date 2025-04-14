// 类型实现Drop特性后，可以让我们自定义当值将要离开作用域时发生的动作，例如文件、网络资源的释放等
// 实现Drop特性只需要实现drop方法，任何类型都可以实现Drop特性
// drop方法的参数是&mut self
struct CustomSmartPointer {
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
        // 出于演示目的，只打印信息，没有做释放动作
    }
}

fn main() {
    let c = CustomSmartPointer {data:String::from("my stuff")};
    let d = CustomSmartPointer {data:String::from("other stuff")};
    println!("CustomSmartPointers created");

    let e = CustomSmartPointer {data:String::from("new stuff")};
    // e.drop();   //   Compiling error, rust不允许手动调用实例的drop方法
    drop(e);    //  可以调用标准库中的 std::mem::drop 函数来提前drop, 且无需担心多次释放的问题，drop是安全的

}   //  c和d实例离开作用域时会自动调用drop方法，并按照先进后出原则依次释放
