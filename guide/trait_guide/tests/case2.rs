mod trait_bound {
    use std::fmt::{Debug, Display};

    trait Summary {}
    trait HelloWorld {}

    // 特质约束定义在泛型参数后
    fn api_1<T: Summary + HelloWorld>(item: &T) {}

    // 特质约束定义在入参后
    fn api_2(item: &(impl Summary + HelloWorld)) {}

    // 特质约束定义在where子句
    fn api_3<T>(item: &T)
    where
        T: Summary + HelloWorld,
    {
    }

    // 对于多种泛型参数具有不同特质约束的复杂情况，通常使用where子句的形式
    fn api_4<T, U>(item1: &T, item2: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }

    impl HelloWorld for i32 {}
    impl HelloWorld for &str {}
    fn api_5() -> impl HelloWorld {0}
    fn api_6() -> Box<dyn HelloWorld> {
        if true {
            Box::new(520)
        } else {
            Box::new("520")
        }
    }
}

mod return_trait {
    trait HelloWorld {}
    impl HelloWorld for i32 {}
    impl HelloWorld for &str {}
    fn api_5() -> impl HelloWorld {0}
    // 配合智能指针，可实现动态派发，返回不同的类型
    fn api_6() -> Box<dyn HelloWorld> {
        if true {
            Box::new(520)
        } else {
            Box::new("520")
        }
    }
}
