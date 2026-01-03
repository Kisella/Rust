fn main() {
    let x = 5;
    match x {
        0 => println!("I'm a zero"),
        y => println!("got y = {}", y),  // y捕获了5，相当于发生了一次 let y = x
    }
}