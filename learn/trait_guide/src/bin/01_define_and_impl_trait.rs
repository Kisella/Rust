trait Swim{
    fn swim(&self);
}

struct Person {
    name: String,
    age: u8,
    can_swim: bool,
}

struct Duck {
    color: String,
}

impl Swim for Person {
    fn swim(&self) {
        match self.can_swim {
            true => println!("I've learned to swim."),
            false => println!("I can't swim.")
        }
    }
}

impl Swim for Duck {
    fn swim(&self) {
        println!("I was born to swim.")
    }
}

fn main() {
    Person {name: String::from("zhangsan"), age: 28, can_swim: true}.swim();
    Duck {color: String::from("yellow")}.swim();
}