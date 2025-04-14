
fn main() {
    println!("Hello Wolrd!");
}

/*
 * case 22 faker
 * 可变引用降级
 */
fn add_big_string(dst: &mut Vec<String>, src: &[String]) {
    // largest对dst内的元素进行了不可变引用，使得dst降级为不可变引用（暂时失去更改权限）
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            // dst.push(s.clone());    //  dst已降级，不能更改
            println!("Compiling error!");
        }
    }
}

/*
 * case 22 truth
 * 避免可变引用降级
 */
fn add_big_string1(dst: &mut Vec<String>, src: &[String]) {
    // 尽量返回值的某些属性而不是返回引用，可避免降级问题
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();    //  diff from case 23
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());    // 没有声明dst的引用，dst可用于修改值
            println!("Compiling error!");
        }
    }
}
