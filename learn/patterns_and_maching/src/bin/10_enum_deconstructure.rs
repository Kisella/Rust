fn main() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::West => println!("West"),
        _ => println!("North or South"),
    }

    enum Message {
        Quit,
        Move(i32, i32),
        Write(String),
    }

    let msg = Message::Move(10, 20);

    match &msg {
        Message::Quit => println!("Quit"),
        Message::Move(x, y) => println!("Move to ({x} {y})"),
        Message::Write(s) => println!("Write: {s}"),
    }

    match &msg {
        Message::Quit => println!("Quit"),
        Message::Move(0, y) => println!("Only move at Y {y}"),
        Message::Move(x, 0) => println!("Only move at X {x}"),
        Message::Move(x, y) => println!("Move to ({x} {y})"),
        Message::Write(s) => println!("Write: {s}"),
    }
}

fn function() {
    enum Message {
        Quit,
        Move{ x: i32, y: i32},
        Write(String),
    }

    let msg = Message::Move{ x: 10, y:20};

    match &msg {
        Message::Quit => println!("Quit"),
        Message::Move{ x, y} => println!("Move to ({x} {y})"),
        Message::Write(s) => println!("Write: {s}"),
    }
}