fn main() {
    fn dosomething<T, U>() {
        println!("sizes: T = {}, U = {}",
            std::mem::size_of::<T>(),
            std::mem::size_of::<U>());
    }

    dosomething::<i32, u8>();
}
