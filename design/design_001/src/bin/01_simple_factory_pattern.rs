/// 简单工厂模式
/// 使用Box<dyn Trait>实现运行时多态
/// 通过match模式匹配创建具体对象

// 定义运算类; 在Rust中使用trait替代
trait Calculator {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64;
}

// 定义加减乘除类，在Rust中使用struct替代，并实现Operation trait
struct Add;
impl  Calculator for Add {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64 {
        number_a + number_b
    }
}

struct Sub;
impl Calculator for Sub {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64 {
        number_a - number_b
    }
}

struct Mul;
impl Calculator for Mul {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64 {
        number_a * number_b
    }
}

struct Div;
impl Calculator for Div {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64 {
        if number_b == 0 {
            panic!("除数不能为0");
        }
        number_a / number_b
    }
}

// 定义运算类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

// 定义工厂类; 在Rust中省略工厂类的定义，直接定义用于生产实例的函数即可
fn create_calculator(operate: OperationType) -> Box<dyn Calculator> {
    match operate {
        OperationType::Add => Box::new(Add),
        OperationType::Sub => Box::new(Sub),
        OperationType::Mul => Box::new(Mul),
        OperationType::Div => Box::new(Div),
    }
}

fn main() {
    let calculator: Box<dyn Calculator> = create_calculator(OperationType::Add);
    let result = calculator.get_result(1, 2);
    println!("结果: {}", result);
}