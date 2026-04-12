// 泛型写法
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

// impl Trait 写法
fn print_debug(x: impl std::fmt::Debug) {
    println!("{:?}", x);
}

fn main() {

}