
/*
 * case 117
 * 当trait中至少存在一个方法具有编译器无法验证的不变量时，该trait就是不安全的
 */

unsafe trait Foo {
    // methods go here
}

// 为某类型实现unsafe类型时，也需要添加unsafe关键字
unsafe impl Foo for i32 {
    // method implementations go here
}

mod test {

}