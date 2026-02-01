mod associated_type {
    use std::fmt::Display;

    trait Processer {
        type Input: Display;
        type Output: Display;
        fn process(&self, input: Self::Input) -> Self::Output;
    }

    struct TextProcessor;
    struct NumberProcessor;

    // 带trait bound的关联类型
    impl Processer for TextProcessor {
        type Input = String;
        type Output = usize;
        fn process(&self, input: Self::Input) -> Self::Output {
            input.len()
        }
    }

    impl Processer for NumberProcessor {
        type Input = i32;
        type Output = f64;
        fn process(&self, input: Self::Input) -> Self::Output {
            (input * 2).into()
        }
        
    }
}
