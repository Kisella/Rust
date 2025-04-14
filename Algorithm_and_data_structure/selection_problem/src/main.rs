use selection_problem::algrithm::*;
fn main() {
    let mut x = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let mut y = [9,4,2,6,1,8,7,3,5];
    println!("{}",select(&mut x, 3).unwrap());
    println!("{}",select(&mut y, 4).unwrap());
}
