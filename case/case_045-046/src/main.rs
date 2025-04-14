
/*
 * case 46
 * blanket 实现
 * 可以为实现了某个trait的类型有条件的实现另外一个trait
 */ 
// 例如在标准库中为实现了Display特性的类型也实现了ToString特性
// impl<T: Display> ToString for T {
//     // --snip--
// }

fn main() {
    println!("case 46");
    let s = 3.to_string();  //  3是整形，整形实现了Display特性，连带着也实现了ToString特性
    println!("{s}");
}
