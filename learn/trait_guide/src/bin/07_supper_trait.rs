use std::fmt::Display;
trait Introduce: Display {
    fn introduce(&self) {
        println!("我是{}", self);
    }
}

struct Cat { name: String}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "猫咪，{}", self.name)
    }
}
impl Introduce for Cat {}

fn main() {
    let cat = Cat{ name: String::from("柴郡")};
    cat.introduce();
}
