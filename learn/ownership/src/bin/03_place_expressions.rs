fn main() {
    let arr = [1, 2, 3];
    let x = arr;    //  局部变量
    let y = arr[1];     //  数组索引

    struct Point { x: i32, y: i32 };    
    let mut p = Point { x: 1, y: 2 };
    let z = p.x;    //  字段方位

    let mut v = 42;
    let r = &mut v;
    *r = 100;   //  解引用

}