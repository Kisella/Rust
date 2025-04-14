fn main() {
    // s 不可用
    let mut s = String::from("Hello");  // s 可用
    // 可以使用String类型中的from函数从字符串字面值创建String类型
    // 这类字符串是可以被修改的，为了支持可变性，String类型需要在heap中分配内存来保存编译时未知的文本内容，这也就意味着操作系统必须在运行时来申请内存，这一步是通过调用String::from来实现的

    s.push_str(", World");  // 可以对s进行相关操作
    println!("{}", s);  
}   //  s 作用域到此结束，调用drop函数将s释放，s不再可用
