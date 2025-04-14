/*
 * case 71
 * 当闭包捕获了值的引用后，值必须保持有效，也不能被移动
 * 闭包可以作为参数传递给函数，也可以作为返回值被函数返回
 */ 
fn make_a_cloner(s_ref:&str) -> impl Fn() -> String {
    || s_ref.to_string()
}

fn main() {
    let s_own =  String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    // drop(s_own);   //  Compiling error, s_own的引用已被捕获，所有权不能被移动或释放
    println!("{s_own}");
    println!("{}", cloner());

    drop(cloner);   //  将cloner销毁后才可移动s_own的所有权
    drop(s_own);    
}
