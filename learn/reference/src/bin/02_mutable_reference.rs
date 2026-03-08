fn main() {
    let mut s: String = String::from("Hello");
    let r1: &mut String = &mut s; // 可变引用
    r1.push_str(", world"); // 通过可变引用修改字符串
    let ref mut r2 = s;
    r2.push_str("!");
    println!("修改后的字符串：{s}");

    let v = 42;
    // let r = &mut v; // 变量未声明为可变，不可取得可变引用
    let mut v = v; // 将 v 声明为可变
    let r = &mut v; // 现在可以取得可变引用
    *r += 1; // 通过可变引用修改值
    println!("修改后的值：{}", *r);

    let mut x = 0;
    let mut y = 1;
    let mut reference: &i32 = &x;
    println!("{reference}");
    reference = &mut y; //   不推荐，reference是可变的，故可以变化绑定为y的引用，但reference的类型已经被确定为不可变的&i32，故这里的mut是无效的
    println!("{reference}");
    // *reference += 1;   //   reference的类型依然是&i32不可变引用，故不能修改y的值
    let reference: &mut i32 = &mut y; //  推荐，重新声明变量接收y的可变引用
    *reference += 1;
    println!("{reference}");
}