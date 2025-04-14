trait Operation {
    fn getResult(numberA: i64, numberB: i64) -> i64 {0}
}

enum Opera {
    Add,
    Sub,
    Mul,
    Dev,
}

struct Add {}
impl Operation for Add {
    fn getResult(numberA: i64, numberB: i64) -> i64 {
        numberA + numberB
    }
}

struct Sub {}
impl Operation for Sub {
    fn getResult(numberA: i64, numberB: i64) -> i64 {
        numberA - numberB
    }
}

struct Mul {}
impl Operation for Mul {
    fn getResult(numberA: i64, numberB: i64) -> i64 {
        numberA * numberB
    }
}

struct Dev {}
impl Operation for Dev {
    fn getResult(numberA: i64, numberB: i64) -> i64 {
        numberA / numberB
    }
}

struct OperationFactory {}
impl OperationFactory {
    fn createOperate(operate: &Opera) -> Box<dyn Operation> {
        match operate {
            Opera::Add => Box::new(Add{}),
            Opera::Sub => Box::new(Sub{}),
            Opera::Mul => Box::new(Mul{}),
            Opera::Dev => Box::new(Dev{}),
        }
    }
}

