// 在没有实现Dedef特性之前，Rust只能解引用一个类型的普通引用。任何类型在实现了Dedef特性后，由于deref方法会返回一个值的引用，就可以使用解引用来获取值了。
// 这种实现了Deref特性的类型，可通过自动调用deref方法来获得值的引用，故就有了类似指针的性质，亦被称作"智能指针"
use std::ops::Deref;

/*
 * case 75
 * 实现了Deref特性的类型可以使用星号(*)进行解引用
 * 实现了Deref特性的类型在解引用时将自动调用deref方法来获取值的引用，进而获取值
 * 这种实现了Deref特性的类型亦被称作"智能指针"
 */ 
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

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    println!("case 75");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // ->  *(y.deref)           自动调用deref的方法
    // ->  *(MyBox(5).deref)
    // ->  *(&5)                 MyBox(5).0 == 5
    // ->  5

    /*
     * case 76
     * Deref coercion 隐式解引用转换
     * 隐式解引用转化能将实现了Deref特质的类型的引用自动转化为另一个类型的引用，以满足参数的要求
     * 例如String类型实现了返回&str的Deref特质，故可以将&String转换为&str
     * 这种转换是由编译器实现的，也可看作是对值调用deref方法后再解引用
     */ 
    println!("\ncase 76");
    let m = MyBox::new(String::from("Rust"));
    hello(&m); //  &MyBox(String) --> &String --> &str
    hello(&(*m)[..]);   //   如果没有隐式转化则需要这样写，可读性差
}

