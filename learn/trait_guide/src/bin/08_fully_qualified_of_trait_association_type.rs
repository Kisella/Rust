trait Pilot {
    type Badge; //  飞行员徽章
}
trait Engineer {
    type Badge; //  工程师徽章
}

fn check_badge<T: Pilot + Engineer>() {
    // let my_badge: T::Badge; //  出现歧义，编译器不知道是那个Badge
    let pilot_badge: <T as Pilot>::Badge;
    let engineer_badge: <T as Engineer>::Badge;
}

fn main() {
    let h = Human;
    h.fly();            //  调用固有方法
    Wizard::fly(&h);     //  trait::method 调用
    <Human as Wizard>::fly(&h);  // 完全限定语法调用
}
