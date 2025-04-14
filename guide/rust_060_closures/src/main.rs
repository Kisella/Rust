/*
 * 闭包必须显式声明捕获引用的声明周期
 */ 

// fn make_a_clonner(s_ref: &str) -> impl Fn() -> String {
//     move || s_ref.to_string()   //  Compiling error, s_ref的生命周期是有限的，而返回类型String又不涉及生命周期，故Rust无法断定该函数是安全的，故报错。
// }

// 我们需要告诉编译器，从make_a_clonner返回的闭包的生命周期不能比捕获的引用s_ref更长，只有这样函数才是安全的
// 标注闭包的生命周期，在闭包末尾使用 + 'a
fn make_a_clonner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {    
    move || s_ref.to_string()   
}

// 借助生命周期省略规则，可以省略生命周期标记，但又不能完全省略，闭包后仍需要加上 + '_ , 表明闭包的生命周期依赖与前面某个引用参数的生命周期
fn make_a_clonner1(s_ref: & str) -> impl Fn() -> String + '_ {    
    move || s_ref.to_string()   
}

fn main() {
    let s_own = String::from("Hello world");
    let cloner = make_a_clonner(&s_own);    //  定义闭包cloner，捕获了s_own
    // drop(s_own);    //  使s_own失效
    cloner();       //  这里cloner捕获的引用已经生效，如果允许不标注生命周期的make_a_clonner这个函数编译通过，就会产生野指针引用的问题
}
