use std::rc::Rc;


fn main() {
    /*
     * case 84
     * 调用Option<T>的Some函数将发生所有权的转移
     * 若不想移动所有权，可选择调用clone方法或者借用的形式
     */ 
    println!("case 84\n");
    let a = 10;
    let some_a = Some(a);
    println!("{a}");
    let b = Rc::new(10);
    let some_b = Some(b);
    // println!("{b}");
    println!("Compiling error");
    let c = Rc::new(10);
    let some_c = Some(c.clone());
    println!("{c}");

    /*
     * case 85
     * unwrap方法会使所有权发生转移
     * 如果不想所有权发生转移，调用unwarp方法前先调用clone方法
     */ 
    println!("case 85\n");
    let a = 10;
    let some_a = Some(a);
    some_a.unwrap();
    println!("{some_a:?}");
    let b = Rc::new(10);
    let some_b = Some(b);
    some_b.unwrap();
    // println!("{some_b:?}");
    println!("Compiling error");
    let c = Rc::new(10);
    let some_c = Some(c);
    some_c.clone().unwrap();
    println!("{some_c:?}");


    /*
     * case 86
     * 使用模式匹配对Option<T>进行解构时会发生所有权的移动
     * 若不想所有权发生移动，可调用clone方法
     * 使用Some(_)进行匹配时，所有权不移动
     */ 
    let a = 10;
    let some_a = Some(a);
    if let Some(a1) = some_a {};
    println!("{some_a:?}");

    let b = Rc::new(10);
    let some_b = Some(b);
    if let Some(b1) = some_b {};    //  此处若不想移动所有权，可调用some_b的clone方法
    // println!("{some_b:?}");
    println!("Compiling error");

    let c = Rc::new(10);
    let some_c = Some(c);
    if let Some(_) = some_c {};     //  使用Some(_)进行解构，所有权不移动
    println!("{some_c:?}");


}
