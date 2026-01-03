fn main() {
    let num = 10;

    match num {
        n if n % 2 == 0 => println!("Even number"),
        n if n % 2 == 1 => println!("Odd number"),
        _ => println!("Something else"),
    }

    let ch = 'x';

    match ch {
        'a'..='z' | 'A'..='Z' if ch != 'x' => println!("Other letter"),
        'x' => println!("The special x"),
        _ => println!("Not a letter"),
    }
}