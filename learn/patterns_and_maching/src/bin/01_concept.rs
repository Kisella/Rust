enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North => println!("North"),
        Direction::South => println!("South"),
        Direction::West => println!("West"),
    };
}