unsafe trait Divide {
    fn div(&self, other: Self) -> Self;
}

unsafe impl Divide for i32 {
    fn div(&self, other: Self) -> Self {
        *self / other   // ❌ 实现者未考虑除零错误
    }
}

fn main() {
    // 调用unsafe trait的普通方法无需unsafe块
    10.div(0);  // 除零导致panic, 责任在与实现者
}


trait SafeTrait {
    fn safe_method(&self);
    unsafe fn unsafe_method(&self);
}

impl SafeTrait for i32 {
    fn safe_method(&self) {
        println!("Safe method called on {}", self);
    }
    
    unsafe fn unsafe_method(&self) {
        println!("Unsafe method called on {}", self);
    }
}