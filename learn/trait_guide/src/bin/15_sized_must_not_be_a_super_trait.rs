// 定义对象
struct Duck;
struct Person;
struct Dog;
// 定义共同行为
trait Swimmer: Sized {
    fn swim(&self);
}

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

// ❌ Swimmer特征被限定为Sized，因此不能用作Trait Object，以下代码将无法编译
// fn make_swim(swimmer: &dyn Swimmer) {
//     swimmer.swim();
// }

fn main() {
    let duck = Duck;
    let person = Person;
    let dog = Dog;

    // make_swim(&duck);
    // make_swim(&person);
    // make_swim(&dog);
}
