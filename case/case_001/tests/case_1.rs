/*
 * case 1
 * 不可变变量可以赋值给可变变量
 * 不可变变量也可以传递给可变参数
 * 同一类型的可变变量与不可变变量之间的类型转化是自动进行的
 */ 
mod case_1 {
    #[test]
    fn case_1() {
        let num1 = 10;
        let mut num2 = num1;    //  不可变变量赋值给可变变量
        num2 += 1;
        let num3 = add_one(num1);   //  不可变变量可以传递给可变参数
        assert_eq!(num2, 11);
        assert_eq!(num3,11);

        let s1 = String::from("Hello");
        let mut s2 = s1.clone();    //  不可变变量赋值给可变变量
        s2.push_str(" World!");
        let s3 = append_world(s1.clone());      //  不可变变量可以传递给可变参数
        assert_eq!(s2, "Hello World!");
        assert_eq!(s3, "Hello World!");
    }

    fn add_one(mut num: i32) -> i32 {
        num = num + 1;
        num
    }
    
    fn append_world(mut s:String) -> String {
        s.push_str(" World!");
        s
    }
    
}