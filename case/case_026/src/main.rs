fn main() {
    /*
     * case 26 
     * 不能简单通过解引用的方式将所有权移动
     * 整形数据不拥有堆数据，可以被直接复制
     * String类型拥有堆数据，如需复制数据需要使用clone方法
     * &String类型(引用类型)不拥有堆数据，可以被直接复制
     */ 
    let v = vec![0, 1, 2];
    let n_ref = &v[0];
    let n =  *n_ref;    //  通过解引用的方式实现值的复制

    let v = vec![String::from("Hello world"),String::from("I love you"),String::from("son of a beach")];
    let s_ref = &v[0];
    // let s = *s_ref;     // 不能简单通过解引用的方式将所有权移动
    println!("Compiling error");
    let s = s_ref.clone();
    let s = v[0].clone();
    println!("{s}");    //  通过克隆以复制在堆上的数据


    let v = String::from("Hello world");
    let s_ref = &v;
    // let s = *s_ref;     // 不能简单通过解引用的方式将所有权移动
    println!("Compiling error");
    let s = s_ref.clone();
    let s = v.clone();
    println!("{s}");    //  通过克隆以复制在堆上的数据
}
