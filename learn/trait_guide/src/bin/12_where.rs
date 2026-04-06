use std::fmt::Display;



trait MyTrait<T, U>
where 
    T: std::ops::Add + std::ops::Neg,
    U: Display + PartialOrd,
    Self: Copy,
{}

fn main() {

}