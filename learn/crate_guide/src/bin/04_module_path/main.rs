fn main() {
    crate::m1::sub_m1::method1();  //    绝对路径
    crate::m2::sub_m2::method2();
    crate_guide::m3::sub_m3::method3(); // 调用library crate中的模块
}

mod m1 {
    pub mod sub_m1 {
        pub fn method1() {
            println!("Method 1");
        }
    }
}

mod m2;    //  模块定义在m2.rs中
