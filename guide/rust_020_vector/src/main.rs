use std::collections::btree_map::Values;

// Vec<T>, 叫做vector, 它由标准库提供，可以存储相同类型的多个值，这些值在内存中连续存放
fn main() {
    let v :Vec<i32> = Vec::new();   //  若只创建一个空vector, rust无法推断出其存储的类型，故需要显示声明
    println!("{:?}",v);

    let mut v = Vec::new();
    v.push(0);  // 使用push方法向vector末尾添加元素
    //  对一个空vector使用push方法后rust就可以推断出类型了，故在声明vector时就不需要显示声明了
    println!("{:?}",v);

    let v = vec![0,1,2,3,4];
    println!("{:?}",v);
    // 使用初始值创建Vec<T>, 使用vec!宏

    // 读取vector中的元素
    let v = vec![1,2,3,4,5];
    let third = &v[2];  //  使用索引来读取元素，若索引超出范围则panic
    println!("The third element is {}", third);

    match v.get(2) {    //  使用get方法和match匹配来读取元素
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // 不能在同一作用域内同时拥有可变引用和不可变引用
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    // v.push(6);     // Complying error
    println!("The first element is {}", first);

    // 遍历vector
    let mut v = vec![100,32,57];
    for ve in &v {
        println!("{}",ve);
    }
    for (index, value) in v.iter().enumerate() {
        println!("vec[{}] is {}. ", index, value);
    }

    for ve in &mut v {
        *ve += 50;   //  使用解引用符号(*)可修改vector里的值
        println!("{}",ve);
    }

    for (index,value) in v.iter_mut().enumerate() {
        *value += 50;
        println!("vec[{}] is {}.
        ", index, value);
    }

}   //  当vector离开作用域后将会被释放
