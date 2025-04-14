use std::rc::*;
fn main() {
    /*
     * case 80
     * Rc引用计数指针
     * 若想使heap上的一个值拥有多个所有者（在栈上有多指针指向同一块内存），可使用Rc指针指针
     * 指向同一个值的不同Rc指针彼此独立，相互间不受影响，只要还存在Rc指向值，值就不会被清理
     * Rc::clone方法，参数为&Rc<_>, 为参数Rc指针所有指向的值拷贝生成一个新的Rc指针
     */ 
    println!("case 80");
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&b);
    println!("{} {} {}", Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c));
    drop(a);
    println!("{} {}", Rc::strong_count(&b), Rc::strong_count(&c));
    // println!("{a}");
    println!("{b}");
    println!("{c}");

    /*
     * case 81
     * Rc引用计数指针
     * 当一个值生成一个指向它的Rc指针时，它的引用计数加1；指向它的Rc指针走出作用域时，引用计数减1
     * 一个值的引用计数变为0时，它的值将被清理
     * Rc::strong_count可查看一个值的引用计数，参数为&Rc<_>
     */ 
    println!("\ncase 81");
    let a = Rc::new(6);
    assert_eq!(Rc::strong_count(&a), 1);
    {
        let b: Rc<i32> = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        let c = a.clone();
        assert_eq!(Rc::strong_count(&a), 3);
    }
    assert_eq!(Rc::strong_count(&a), 1);

}
