#[allow(arithmetic_overflow)]
fn main() {
    let l = std::usize::MAX;
    let r = 6;
    println!("{}", l.wrapping_add(r));
}
