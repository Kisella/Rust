use longest_common_substring::algrithm::*;
fn main() {
    let x = ['A', 'B', 'C', 'A', 'D', 'B', 'B'];
    let y = ['B', 'C', 'E', 'D', 'B', 'B'];
    let lcstr = longest_common_substring(&x, &y);
    println!("{x:?} \n {y:?}");
    println!("The lcs between x and y is {lcstr:?}");
    println!("Hello, world!");
}
