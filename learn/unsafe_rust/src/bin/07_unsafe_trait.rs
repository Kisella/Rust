unsafe trait Divide {
    fn div(&self, other: Self) -> Result<Self, &'static str> where Self: Sized;
    unsafe fn div_uncheck(&self, other: Self) -> Self;
}

unsafe impl Divide for i32 {
    fn div(&self, other: Self) -> Result<Self, &'static str> {
        if other == 0 {
            Err("Division by zero!")
        } else {
            Ok(*self / other)
        }
    }


    unsafe fn div_uncheck(&self, other: Self) -> Self {
        *self / other   // 调用者自己注意
    }
}

fn main() {
    unsafe {
        10.div_uncheck(10);
    }
}