fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);

    let s3 = String::from("hello");
    let len = calculate_length_by_reference(&s3); //  s3的所有权并未发生移动
    println!("The length of '{}' is {} ", s3, len);

    let mut s4 = s3;
    append_world(&mut s4);
    println!("{}", s4);

    let mut s = String::from("Hello");
    let s1 = &mut s;
    // let s2 = &mut s;     //  Compling failure
    // 在特定的作用域内，对于同一块数据只能有一个可变的引用。这样做的好处是在编译时就防止数据竞争的发生

    {
        let s1 = &mut s;
    }
    let s2 = &mut s; 
    // 可以通过创建新的作用域来允许"非同时的"创建多个可变引用

    let r1 = &s;
    let r2 = &s;
    // let r3 = $mut s;         //  Compling failure
    // println!("{} {} {}", r1, r2, r3);
    // 不可以同时存在对同一块数据的可变引用和不可变引用
}

//  在本例中，calculate_length获取了s1的所有权，又将所有权通过返回值还会给main, 目的仅仅只是为了获取s1中的所有权。不免有些繁琐
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// &符号表示'引用', 允许你引用某些值而不取得其所有权
fn calculate_length_by_reference(s: &String) -> usize {
    s.len()         //  Rust会自动识别并解引用
}

fn append_world(s: &mut String) {
    s.push_str(", world!");        
}
// 和变量一样，应用默认也是不可变的, 可通过增加mut关键使引用可变
