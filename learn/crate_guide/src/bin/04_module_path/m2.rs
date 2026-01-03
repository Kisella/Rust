pub mod sub_m2 {
    pub fn method2() {
        println!("method2 call method1");
        super::super::m1::sub_m1::method1();   // 相对路径
    }
}