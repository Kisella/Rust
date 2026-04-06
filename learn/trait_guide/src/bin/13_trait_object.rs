// 定义对象
struct Duck;
struct Person;
struct Dog;

// 定义共同行为
trait Swimmer {
    fn swim(&self);
}

// Duck, Person, Dog都实现了Swimmer特质，所有都属于抽象类型&dyn Swimmer
impl Swimmer for Duck {
    fn swim(&self) {
        println!("Duck paddles through the pond.");
    }
}
impl Swimmer for Person {
    fn swim(&self) {
        println!("Person swims freestyle.");
    }
}
impl Swimmer for Dog {
    fn swim(&self) {
        println!("Dog does the doggy paddle.");
    }
}

// 定义函数，接受一个Trait Object作为参数参数
fn make_swim(swimmer: &dyn Swimmer) {
    swimmer.swim();
}

fn main() {
    let duck = Duck;
    let person = Person;
    let dog = Dog;

    make_swim(&duck);
    make_swim(&person);
    make_swim(&dog);
}