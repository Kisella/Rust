use minimum_edit_distance::algrithm::*;
fn main() {
    let s = "";
    let t = "";
    let (mim_oprea, opera) = minmum_edit_distance(s, t);
    println!("将{s}编辑为{t}, 需要的最少操作为{mim_oprea}");
    println!("操作如下:");
    for item in opera.iter() {
        println!("{item}");
    }
}