fn main() {
    let score = 77;
    match score {
        0..=59 => println!("Fail"),
        60..=89 => println!("Good"),
        90..=100 => println!("Excellent"),
        _ => println!("Invalid score"),
    }
}