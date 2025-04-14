/*
 * case 98
 * 单元测试————带有#[test]属性注解的函数
 * cargo test会构建test runner二进制文件，运行带有#[test]属性的函数, 并报告结果
 */
fn one_plus_one() -> usize {
    1 + 1
}

#[test]
fn test_one_pluse_one() {
    let result = one_plus_one();
    assert!(result == 2);
}

/*
 * case 99
 * 测试失败
 * 测试函数中通常有断言(assert), 当断言被触发时通常意味着测试失败
 */
#[test]
fn fail_testfail_test() {
    panic!("Make this test fail");
}

/*
 * case 100
 * 测试模块
 * 使用#[cfg(test)]可以声明一个测试模块
 * 使用"cargo test 测试模块名"可以运行测试模块中所有带有#[test]注解的测试函数
 * #[cfg(test)]确保测试代码仅在运行cargo test时编译和运行，而不会影响普通的构建过程
 */
#[cfg(test)]
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

/*
 * case 101
 * 断言宏
 * assert!宏接收一个bool值作为参数，为true无事发生，false则恐慌
 * assert_eq!宏接收两个相同类型的值作为参数，两值相等无事发生，不等则恐慌
 * assert_ne!宏接收两个相同类型的值作为参数，两值相等无事发生，不等则恐慌
 */
#[test]
fn test_assert() {
    assert!(true);
    assert_eq!(1 + 1, 2);
    assert_ne!("white horse", "horse");
}



/*
 * case 102
 * #[should_panic]
 * #[should_panic]放在#[test]属性下边，当测试函数发生恐慌时测试通过，否则(函数未恐慌)测试将失败
 */
#[test]
#[should_panic]
fn test_should_panic() {
    panic!("This test panic but passed!")
}

/*
 * case 103
 * #[should_panic()]
 * #[should_panic]可以带参数，若test函数未发生恐慌或恐慌信息不包含入参文本的话，测试将失败
 */
#[cfg(test)]
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

/*
 * case 104
 * 测试函数也可以使用Result<T, E>作为返回类型
 * 当返回的是Err变体时，测试将失败
 * 使用Result<T, E>的好处时可以使用?来简化测试代码
 * 使用Result<T, E>作为返回类型的测试函数不能使用should_panic属性
 */
#[test]
fn test_return_err() -> Result<(), &'static str> {
    Err("this test fail!")
}

/*
 * cargo test -- --show-output  显示测试函数的打印内容
 * cargo test 测试函数名或模块名   指定测试的函数或模块
 * cargo test 模糊搜索   可指定测试带有指定字符的函数和模块
 *     -例如 cargo test should 就会运行test_should_panic函数和test_should_panic_mod模块的测试        
 */ 


/*
 * case 105
 * 带有#[ignore]标识在运行cargo test时不会运行
 * cargo test -- --ignored 会运行标记了#[ignore]的测试
 * cargo test -- --include-ignored  会运行所有测试
 */ 
#[cfg(test)]
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