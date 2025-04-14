use maxtrix_chain_multiply::algrithm::*;

fn main() {
    let chain_case = [(2, 3), (3, 7), (7, 9), (9, 5), (5, 2), (2, 4)];
    let matrix_chain = Matrix::new_chain(&chain_case);
    let (min, cutting) = matrix_chain_multiply(&matrix_chain);
    println!("{min} \n {cutting}");

    println!("Hello, world!");
}
