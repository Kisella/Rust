struct Person {
    name: String,
    age: u8,
    can_swim: bool,
}

struct Duck {
    color: String,
}

trait Swim{
    fn swim(&self) {
        println!("I was born to swim.")
    }
}

impl Swim for Duck {} // 使用默认实现

impl Swim for Person {
    fn swim(&self) {   // 覆盖实现
        match self.can_swim {
            true => println!("I've learned to swim."),
            false => println!("I can't swim.")
        }
    }
}



fn main() {

}



