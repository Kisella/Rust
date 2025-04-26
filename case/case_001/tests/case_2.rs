/*
 * case 2 
 * stack上的值赋值给另一个变量是会发生值的复制，动作发生后原变量依然有效
 */
mod case_2 {
    #[test]
    fn case_2() {
        let num1 = 10;
        let num2 = num1;
        assert_eq!(num1, 10);
        assert_eq!(num2, 10);
    }
}

/*
 * case 3 反例
 * heap上的值赋值给另一个变量是会发生所有权的移动，动作发生后原变量失效
 */ 
mod case_3 {
    #[test]
    fn case_3() {
        let s1 = String::from("Hello");
        let s2 = s1;
        assert_eq!(s2, "Hello");
        // println!("s1: {s1}");                //  使用了失效后的变量
        println!("Compiling error");
    }
}

mod case_4 {
    #[test]
    fn case_4() {
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        assert_eq!(s1, "Hello");
        assert_eq!(s2, "Hello");
    }
}