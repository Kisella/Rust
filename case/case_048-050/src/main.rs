/*
 * 在一个函数中的声明的使用只能在内部使用，而不能返回，否则就会造成悬垂引用。故可以得出，能做为返回值的返回的引用必然来自函数入参，其生命周期来源也必然来自函数入参。当入参有多个时，就需要生命周期参数来标识返回值的引用到底来源与那个入参引用
 */ 

/*
 * case 48 faker
 * 函数返回的 &str 引用必须使用生命周期参数来标识来源 ,否认编译器无法推断返回引用是来源于x还是y
 */
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

/*
 * case 48 truth
 * 生命周期参数由但单分号(')和几个简短字母(通常小写)组成
 * 生命周期参数需要在尖括号(<>)中声明才能使用，声明放在函数名称和参数列表之间
 * 生命周期参数使用时放在&后面，并用空格与引用类型隔开
 */

// 这里返回值的来源可能是x或y，故给x和y都加上了生命周期参数
fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("case 48");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest1(string1.as_str(), string2);
    println!("The longest string is {result}");

    /*
     * case 49 反例
     * 不能使用可能失效的引用
     */ 
    println!("\ncase 49");
    let string3 = String::from("abcd");
    let string4 = String::from("xyz");
    let result = longest1(&string3, &string4);
    println!("The longest string is {result}");
    drop(string4);  //  这里string4失效，而result可能来源于string3或string4，故result在编译器眼中是“可能失效的”。不确定就代表了不安全，故Rust的保守性会拒绝result的使用
    // println!("The longest string is {result}"); //  Compiling error, 使用了可能失效的引用。

    /*
     * case 50 反例
     * 不能使用可能失效的引用
     */ 
    println!("\ncase 50");
    let string5 = String::from("long string is long");
    let result;
    {
        let string6 = String::from("xyz");
        result = longest1(&string5, &string6);
        println!("The longest string is {result}");
    }   //  string6作用域结束，string6失效，result可能失效。
    // println!("The longest string is {result}"); //  Compiling error, 使用了可能失效的引用。
}



