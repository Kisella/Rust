
fn main(){
    let s1 = String::from("hello");
    let s2 = s1;  // 值上下文 + 位置表达式 + 未实现Copy特质 -> 移动

    let s1 = String::from("hello");
    s1;     //  表达式语句，会发生移动，值直接被丢弃，不推荐。推荐使用drop(s1);
}