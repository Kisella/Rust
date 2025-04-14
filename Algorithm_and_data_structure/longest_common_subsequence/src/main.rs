use longest_common_subsequence::algrithm::longest_common_subsequence;
fn main() {
    let x = ['A','B','C','B','D','A','B'];
    let y = ['B','D','C','A','B','A'];
    let lcs = longest_common_subsequence(&x, &y);
    println!("{x:?} \n {y:?}");
    println!("The LCS between x and y is {:?}.", lcs.unwrap());
}
