/// 简单工厂模式, Rust风格改进
/// 使用枚举将不同实例包装成一个统一的类型
/// 工厂函数直接返回具体类型，静态分发，零成本抽象

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

// Rust风格改进，使用基于枚举的多态（Enum-based Polymorphism）
// 实现零成本的静态分发
enum Operation {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
}
impl Calculator for Operation {
    fn get_result(&self, number_a: i64, number_b: i64) -> i64 {
        match self {
            Operation::Add(op) => op.get_result(number_a, number_b),
            Operation::Sub(op) => op.get_result(number_a, number_b),
            Operation::Mul(op) => op.get_result(number_a, number_b),
            Operation::Div(op) => op.get_result(number_a, number_b),
        }
    }
}

//  Rust风格改进，工厂函数直接返回具体类型，静态分发，零成本抽象
fn create_calculator(op: OperationType) -> Operation {
    match op {
        OperationType::Add => Operation::Add(Add),
        OperationType::Sub => Operation::Sub(Sub),
        OperationType::Mul => Operation::Mul(Mul),
        OperationType::Div => Operation::Div(Div),
    }
}

fn main() {
    let calculator = create_calculator(OperationType::Add);
    let result = calculator.get_result(1, 2);
    println!("结果: {}", result);
}