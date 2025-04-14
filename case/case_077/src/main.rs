struct CustomSmartPointer {
    data: String,
}
/*
 * case 77
 * Drop特征定义一个值走出作用域时的清理行为
 * Drop特性只要求实现drop方法，参数是&mut self
 * 在一个值走出作用域时，Rust将自动调用值的drop方法进行值的清理
 * 当一个存有heap数据的值没有实现Drop特性时，Rust编译器也会自动插入Drop特性的实现，避免内存泄漏
 */ 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoiner with data `{}`!", self.data);
    }
}

fn main() {
    println!("case 077");
    
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
// c 和 d 走出作用域，将自动调用drop方法。后声明的值先清理，先声明的值后清理。
