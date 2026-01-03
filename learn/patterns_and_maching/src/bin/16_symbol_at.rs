fn main() {
    struct Ticket {
        used: bool,
        owner: String,
    }

    impl Ticket {
        fn consume(&mut self) {
            self.used = true
        }
    }

    let t = Ticket {
        used: false,
        owner: String::from("Jack"),
    };

    // 变量t移动到了新的变量tic上，并被声明为可变
    match t {
        mut tic @ Ticket {used: false, ..} => tic.consume(),
        _ => println!("Nothing"),
    }

    let mut t = Ticket {
        used: false,
        owner: String::from("Jack"),
    };

    match &mut t {
        tic @ Ticket {used: false, ..} => tic.consume(),
        _ => println!("Nothing"),
    }

    match t {
        ref mut tic @ Ticket {used: false, ..} => tic.consume(),
        _ => println!("Nothing"),
    }

    let arr = [1, 2, 3, 4, 5];
    match arr {
        [first, mid @ .., last] => println!("{first} {mid:?} {last}"),
    }
}