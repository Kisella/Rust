fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // let r1 = &s1; // s1被标记为moved, 不能借用s1
    let r2 = &s2; // s2拥有所有权，可以借用s2
    println!("{r2}");

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let r1 = &s1;
    let r2 = &s2;
    println!("{r1} {r2}");

    let s3 = s1;
    // println!("{r1}"); // r1借用s1, s1被移动, r1不再有效
    drop(s2);
    // println!("{r2}"); // r2借用s2, s2被释放, r2不再有效
}