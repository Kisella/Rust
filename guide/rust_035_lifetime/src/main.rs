use std::fmt::Display;

/*
 * 输入生命周期: 在函数、方法的参数上的生命周期
 * 输出生命周期: 在返回值上的生命周期
 *
 * 生命周期省略规则:
 * 1. 编译器会为每个输入类型的参数分配不同的生命周期
 * => fn foo(x: &i32) ------- fn foo<'a>(x: &'a i32)
 * => fn foo(x: &i32, y: &i32) ------ fn<'a,'b>(x: &'a i32, y: &'b i32)
 * => fn foo(x: &ImportantExcerpt) ------ fn foo<'a,'b>(x: &'a ImportantExcerpt<'b>)
 *
 * 2. 如果只有一个输出生命周期参数，该生命周期将被分配给所有的输出生命周期参数
 * => fn foo<'a>(x: &'a i32) -> &'a i32
 *
 * 3. 输入生命周期其中有一个是&self 或 &mut self(因为这是一个方法), 那么self的生命周期会被分配给所有的输出生命周期参数
 */
fn main() {
    //  静态生命周期 'static : 它表示受影响的引用可以在整个程序的持续时间内存活
    let s: &'static str = "I have a static lifetime"; //  所有的字符串字面量都具有'static生命周期

    static NUMBER_S: i32 = 150;
    let s: &'static i32 = &NUMBER_S;
    println!("{s}");
}

/*
 *    fn first_word(s: &str) -> &str {}
 * => fn first_word<'a>(s: &'a str) -> &str {}      应用规则1
 * => fn first_word<'a>(s: &'a str) -> &'a str {}   应用规则2
 *    到此函数first_word中所有的引用都有了生命周期注解，故不需要程序员手动显式注解
 */
/*
 *    fn longest(x: &str, y: &str) -> &str {}
 * => fn longest<'a,'b>(x: &'a str, y: &'b str) -> &str {}   应用规则1
 * 不符合规则2和规则3, 输出生命周期无法确定，故需要程序员手动显式注解生命周期
 */

//  方法定义中的生命周期注解
struct ImportantExcerpt<'a> {
    //  生命周期注解<'a>是结构体类型(定义)的一部分
    part: &'a str,
}

// 为结构体实现方法时，结构体的生命周期参数在impl关键字后进行声明，然后再在结构体名称后使用它
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        //  应用规则1，无需显式声明生命周期
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        //  应用规则1和规则3，无需显式声明生命周期
        println!("Attention please: {announcement}");
        self.part
    }
}

// 泛型类型参数、Trait Bound 和生命周期一起使用的例子
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x  
    } else {
        y
    }
}
