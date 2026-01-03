trait MyAdd<I, O> {
    fn my_add(self, other: I) -> O;
}

impl MyAdd<i32, i32> for i32 {
    //  i32 + i32 -> i32
    fn my_add(self, other: i32) -> i32 {
        self + other
    }
}

impl MyAdd<i64, i64> for i32 {
    // i32 + i64 -> i64
    fn my_add(self, other: i64) -> i64 {
        self as i64 + other
    }
}
// 在不同情况下，根据操作数（参数）来决定返回类型

// i32 + i32 -> 64
impl MyAdd<i32, i64> for i32 {
    // i32 + i64 -> i64
    fn my_add(self, other: i32) -> i64 {
        (self + other) as i64
    }
}

fn main() {
    let ret: i32 = 1.my_add(10);
    let ret = 1.my_add(10_i64);
}