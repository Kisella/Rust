trait MySwap<T> {
    fn my_swap(self, other: T) -> (T, Self);
}
impl<T> MySwap<T> for i32 {
    fn my_swap(self, other: T) -> (T, Self) {
        (other, self)
    }
}

fn main() {

}