pub mod my_models;
fn main() {
    let y = my_models::enums::YesNo::YES;
    println!("{y:?}");
}