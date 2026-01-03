fn main() {
    struct Point<T> {
        x: T,
        y: T,
    }

    // 为所有的Poin<T>类型实现方法
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // 为Point<i32>类型实现方法
    impl Point<i32> {
        fn print(&self) {
            println!("x: {}, y: {}", self.x, self.y);
        }
    }

    // impl<T, TT> Point<T, U> {
    //     // 泛型参数<V, W>只在方法mixup内使用
    //     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    //         Point {
    //             x: self.x,
    //             y: other.y,
    //         }
    //     }
    // }
}

