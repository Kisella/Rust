fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), &string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // string2的存活时间比string1短, 故result会在string2存活期间保持有效
        println!("The longest string is {result}");
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    println!("The longest string is {result}"); //  Compiling error
                                                //    string2死亡后, result将失效
}

// fn longest(x: &str, y: &str) -> &str {  //  rust不知道返回的&str的生命周期应对照x还是y, 故需要显式指明生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //  这个函数签名表明，对于某个生命周期<'a>, 两个参数x和y至少得活的和'a一样长, 返回值&str同理。
    //  这样做意味着函数返回值的引用&str的生命周期和参数x和y中"比较短"的哪一个相同
    //  当生命周期的关系指定后，借用检查器就会拒绝不符合这个约束的值
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 生命周期注解不会改变引用存活的时间，只是为了描述多个引用之间的生命周期关系
// 生命周期参数以单引号(')开头, 后跟简短的小写字母，位置紧跟在&的后边，用空格与引用类型分开
// 例如:    &i32     &'a i32     &'a mut i32

//  从函数返回一个引用时，返回类型的生命周期参数需要与其中一个参数的生命周期参数匹配
//  否则这个引用必然指向在该函数中创建的一个值，进而导致悬垂引用
// fn longest1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("ready long string");
//     result.as_str()     // Compiling error
// }
