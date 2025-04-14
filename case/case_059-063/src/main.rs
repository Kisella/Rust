/*
 * case 59
 * 当返回值和入参中引用的个数都只有一个时，编译器可以直接推断其来源，无需声明生命周期
 */
fn foo(a: &i32, b: String) -> &i32 {
    a
}

/*
 * case 60
 * 当返回值和入参中引用的个数都只有一个时，编译器可以直接推断其来源，无需声明生命周期
 */
#[derive(Debug)]
struct Foo1<'a> {
    bar: &'a String,
}
// Foo1 中只有一个引用字段，故可以省略生命周期参数的声明
fn baz1(f: Foo1, b: String) -> &String {
    f.bar
}

/*
 * case 61
 * 若函数入参中的引用超过一个时，必须在函数签名中声明生命周期参数
 */
struct Foo2<'a, 'b> {
    bar1: &'a String,
    bar2: &'b String,
}
// Foo2 中存在超过一个的引用字段，必须在函数签名中进行生命周期参数的声明
fn baz2<'a, 'b>(f: Foo2<'a, 'b>, b: String) -> &'a String {
    f.bar1
}

/*
 * case 62
 * 当函数入参存在结构体的引用，且这个结构体中含有引用字段时，则认为入参中的中的引用超过一个
 * 必须在函数签名中声明生命周期参数
 */
// fn baz3(f: &Foo1, b: String) -> &i32 {f.bar}  // Compiling error!, &Foo和 Foo.bar和是两个不同的引用, 需要为函数声明生命周期参数
fn baz4<'a, 'b>(f: &'b Foo1<'a>, b: String) -> &'b Foo1<'a> {
    f
}
fn baz5<'a, 'b>(f: &'b Foo1<'a>, b: String) -> &'a String {
    f.bar
}
fn baz6<'a, 'b>(f: &'b Foo1<'a>, b: String) -> &'b String {   //  这里低估了生命周期长度，实际返回的仍然是&'a String，Rust 允许开发者"低估"返回值的实际存活时间，低估的生命周期将被编译器自动修正成匹配的值
    f.bar
}
fn main() {
    /*
     * case 63
     * Rust 允许开发者"低估"返回值的实际存活时间，但禁止"高估"
     */
    let mut num = String::from("1109");
    let f = Foo1 { bar: &num };
    let num2: &String;
    let num3: &String;
    {
        let ref1 = &f;
        num2 = baz5(ref1, String::from("case1"));
        num3 = baz6(ref1, String::from("case1"));   // 按照函数签名num3的生命周期应该与ref1相同
    }
    // println!("{ref1:#?}");
    println!("{num2}");
    println!("{num3}");      //  这里ref1已失效，但num3依然有效，原因是num的生命周期'a完全覆盖ref1的'b，编译器自动修正了num3的生命周期，故操作仍然是安全的。
    //  生命周期协变规则，'a:'b（'a 比 'b 存活更久）时，&'a T 可以协变为 &'b T
}
