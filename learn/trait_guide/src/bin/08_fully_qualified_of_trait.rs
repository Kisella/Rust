trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying the plane!");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying with magic!");
    }
}
impl Human {
    fn fly(&self) {
        println!("human flapping arms... not very effective.");
    }
}

fn main() {
    let h = Human;
    h.fly();            //  调用固有方法
    Wizard::fly(&h);     //  trait::method 调用
    <Human as Wizard>::fly(&h);  // 完全限定语法调用
}
