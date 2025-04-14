use rod_cutting_problem::algrithm::*;
fn main() {
    let p = [0,1,5,8,9,10,17,17,20,24,24];
    let (max, scheme) = rod_cutting(&p);
    println!("钢条长度为{}", p.len() - 1);
    println!("钢条价格表为{p:?}");
    println!("最大收益为{max}");
    println!("切割方案: {scheme:?}");
}
