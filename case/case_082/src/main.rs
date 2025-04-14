use std::cell::RefCell;

fn main() {
    /*
     * case 82 
     * RefCell, 内部可变性指针
     * RefCell指针可以改变指向的值
     */ 
    println!("case 81");
    let a = RefCell::new(10);
    let mut a_ref1 = a.borrow_mut();
    *a_ref1 = 15;
    drop(a_ref1);
    println!("{a:?}");

    /*
     * case 82
     * borrow_mut()方法可以生成值的RefMut引用，通过这个引用可以修改值
     * 值的RefMut存活期间，值不能被读取，也不能够生成其他的引用，否则将发生恐慌
     */ 
    println!("\ncase 82");
    let a = RefCell::new(10);
    println!("{a:?}");
    let a_ref1 = a.borrow();
    println!("{a:?}");
    drop(a_ref1);       //  生成a的可变引用前需将的它其他引用全部丢弃，否则将恐慌
    let mut a_ref2 = a.borrow_mut();
    println!("{a:?}");   //  值的RefMut引用存在，不能读取
    *a_ref2 = 666;
    drop(a_ref2);
    println!("{a:?}");  //  值的RefMut引用灭亡，值可被重新读取
}
