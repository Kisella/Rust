fn main() {
    let mut v = 42;
    let r1: &mut i32 = &mut v;
    *r1 += 1;
    let r2: &i32 = &v;  //  共享借用，可变引用r1失效
    // *r1 += 1;        //  r1 不可再使用

    let mut v = 42;
    let r = &mut v;
    *r += 1;
    let a = v;  //  v的值被读取用于按位拷贝生成a, 它的可变引用r失效
    // println!("{r}");    //  r 不可再使用

    let mut v = 42;
    let r = &v;
    println!("{r}");
    let a = v;
    println!("{r}");    //  值被读取后原有的共享引用不受影响


    let mut v: i32 = 42;
    let r1: &i32 = &v;
    let r2: &i32 = &v;
    let r3: &i32 = &v;
    println!("{r1} {r2} {r3}");
    let r4: &mut i32 = &mut v;  // 可变借用，所有共享引用失效
    // println!("{r1} {r2} {r3}");  //  r1 r2 r3 不可再使用
    *r4 += 1;
    let r5: &mut i32 = &mut v;  // 可变借用，之前的可变引用r4失效
    // *r4 += 1;    //  r4 不可再使用

    let mut v: i32 = 42;
    let r1: &i32 = &v;
    let r2: &i32 = &v;
    let r3: &i32 = &v;
    println!("{r1} {r2} {r3}");
    v += 1;  // 值修改，所有共享引用失效
    // println!("{r1} {r2} {r3}");  //  r1 r2 r3 不可再使用
    let r4: &mut i32 = &mut v;
    println!("{r4}");
    v += 1;  // 值修改，可变引用失效
    // println!("{r4}");  //  r4 不可再使用

    let mut s1 = String::from("hello");
    let r1: &String = &s1;
    println!("{r1}");
    let r2: &String;
    {
        let mut s2 = s1;
        // println!("{r1}");   //  s1被移动，r1不可再使用
        r2 = &s2;
        println!("{r2}");
    }
    // println!("{r2}");   //  s2作用域结束，r2不可再使用
}