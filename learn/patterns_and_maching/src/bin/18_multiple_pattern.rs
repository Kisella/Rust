fn main() {
    let score = 80;

    match score {
        0..40 | 95..=100 => println!("Focus on"),
        40..95 => println!("Normal"),
        _ => println!("Invalid score"),
    }
}