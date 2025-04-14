fn main() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}!", hello, world);
    // 字符串切片：指向字符串(String)中一部分内容的引用
    // 形式：[开始索引..结束索引]，左闭右开，不包括结束索引
    //     - 开始索引就是切片起始位置的索引值
    //     - 结束索引是切片终止位置的下一个索引值

    let hello = &s[..5];    //  语法糖: 开始索引为0时可以省略
    let world =&s[6..s.len()];
    println!("{}, {}!", hello, world);
    let world = &s[6..];    //  语法糖: 结束索引为字符串长度时可以省略
    println!("{}, {}!", hello, world);

    let whole = &s[..];   //  语法糖: 指向整个字符串切片可以用[0..s.len()], 省略为[..]
    println!("{}", whole);

    let hello_slice = first_world(&s);  //  let hello_slice = first_world(&s[..]); 不需要进行切片直接传&String也可以, 效果相同
    println!("{}", hello_slice);

    let hello_world_slice = "Hello World";
    let hello_slice = first_world(hello_world_slice);
    println!("{}", hello_slice);

    //  数组切片
    let a = [1,2,3,4,5];
    let array_slice = &a[1..3];
    println!("array_slice: {:?}", array_slice);
    println!("The len of array_slice is {}", array_slice.len());
}

//  fn first_world(s:&String) -> &str {     // 相比与&String, 通常会采用&str作为参数类型
    fn first_world(s:&str) -> &str {
    let bytes = s.as_bytes();   //  字符串转化为字节数组
    for (i, &item) in bytes.iter().enumerate() {    //  使用.iter().enumerate()获取索引和元素
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
// 相比与&String, 通常会采用&str作为参数类型, 因为这个就可以同时接受String和&str类型的参数, 这使得API更加通用, 且不会损失任何功能
