trait MyAdd<I> {
    type O;
    fn my_add(self, other: I) -> Self::O;
}

impl MyAdd<i32> for i32 {
    type O = i32;
    fn my_add(self, other: i32) -> Self::O {
        self + other
    }
}

impl MyAdd<i64> for i32 {
    type O = i64;
    fn my_add(self, other: i64) -> Self::O {
        self as i64 + other
    }
}

fn main() {

}