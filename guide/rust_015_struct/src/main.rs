#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
// 使用struct关键字为整个struct类型命名
// 在花括号内为所有字段(Field)定义名称和类型，每个字段后边都要有逗号

// 使用impl关键字为struct定义自己特有的的关联函数和方法
// 每个struct允许拥有多个impl块
impl Rectangle {
    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { //  当字段名与字段值对应变量名相同时，就可以使用字段初始化的简写方式
            width,  //  width:width, 简写
            length,
        }   
    }

    fn new_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
    // impl块里可以定义struct的关联函数，关联函数不把self作为函数的第一个参数
    // 关联函数通常用于构造器(创建struct的一个或多个实例)
    // 使用struct类型名加两冒号(::)的形式来调用关联函数

    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
            || self.width >= other.length && self.length >= other.width
    }
    // impl块里可以定义struct的方法，方法和函数类似, fn关键字、名称、参数、返回值的形式与函数相同
    // 方法的第一个参数必须是self(&self, &mut self), 表示方法被调用的struct实例
    // 方法的第一个参数可以是&self, 也可以获得其所有权或可变借用
    // Rust中使用句点(.)来调用方法, 在调用方法时Rust会根据情况自动添加&,&mut或*, 以便object可以匹配方法的签名
}

fn main() {
    let mut rect1: Rectangle = Rectangle {
        length: 50,
        width: 30,
    };
    println!("{:?}", rect1);
    println!("{} {}", rect1.width, rect1.length);
    rect1.width = 45;
    println!("{:?}", rect1);
    // 创建struct实例时，要为每个字段指定具体的值，但可以无需按声明的顺序进行指定
    // 使用句点(.)对struct实例中的字段进行访问
    // 一旦struct的实例是可变的，那么实例中所有的字段都是可变的

    let rect2 = Rectangle::new(10, 40);     //  通过调用关联函数来创建实例
    println!("{:?}", rect2);

    let rect3 = Rectangle::new(35, 55);
    println!("{:#?}", rect3);

    let squr = Rectangle::new_square(25);
    println!("{:#?}", squr);

    println!("The area of rect1 is {}", rect1.area());
    println!("The area of rect2 is {}", rect2.area());
    println!("The area of rect3 is {}", rect3.area());
    println!("The area of squr is {}", squr.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("{}", rect1.can_hold(&squr));
}
