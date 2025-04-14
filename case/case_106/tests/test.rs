use case_106::one_plus_one;
/*
 * case 106
 * 集成测试
 * Rust会将tests目录里的文件区别对待, 只有在运行cargo test时才会编译运行里面的文件
 * test目录里每一个文件都是一个独立的crate(单元测试)
 * 针对集成测试，测试模块不需要添加#[cfg(test)]注解
 * cargo test --test 单元测试文件名称  指定运行特定的单元测试文件
 */ 



#[test]
fn test_one_pluse_one() {
    let result = one_plus_one();
    assert!(result == 2);
}
#[test]
fn fail_testfail_test() {
    panic!("Make this test fail");
}

mod a_test_mod {
    #[test]
    fn test1() {
        println!("test1 call fn_but_not_test and passed");
        fn_but_not_test();
    }

    // 此函数没带有#[test]，不视作一个测试
    fn fn_but_not_test() {
        println!("This is not a test!");
    }

    #[test]
    fn test3() {
        panic!("test3 fail!");
    }
}

#[test]
fn test_assert() {
    assert!(true);
    assert_eq!(1 + 1, 2);
    assert_ne!("white horse", "horse");
}

#[test]
#[should_panic]
fn test_should_panic() {
    panic!("This test panic but passed!")
}

mod test_should_panic_mod {
    #[test]
    #[should_panic(expected = "hello world!")]
    fn test_should_panic_with_parameter_and_fail() {
        panic!("This test panic but fail!")
    }

    #[test]
    #[should_panic(expected = "pass")]
    fn test_should_panic_with_parameter_and_passed() {
        panic!("This test panic and passed!")
    }
}

#[test]
fn test_return_err() -> Result<(), &'static str> {
    Err("this test fail!")
}

mod tests {
    #[test]
    fn test_normal() {
        println!("This test will run.");
    }

    #[test]
    #[ignore = "reason"]
    fn test_ignore() {
        println!("This test will not run by default.");
    }
}
