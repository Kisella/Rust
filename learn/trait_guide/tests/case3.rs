mod impl_trait_for_generics {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    // 为所有的Pair类型实现方法
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        fn x(&self) {}
    }

    // 为实现了Display和PartialOrd特质的Pair类型实现方法
    impl<T:Display + PartialOrd> Pair<T> {
        fn cmp(&self) {}
    }

    // 为f32类型的Pair实现方法
    impl Pair<f32> {
        fn y(&self) {}
    }

    #[test]
    fn test() {
        let pair1 = Pair::new((1, 2), (3, 4));
        let pair2 = Pair::new(8, 3);
        let pair3 = Pair::new(9.5, 3.14);
    
        pair1.x();
        pair2.x();
        pair3.x();  //  所有的pair实例都有x方法
    
     // pair1.cmp();   // pair1没有com_display方法
        pair2.cmp();
        pair3.cmp();
    
     // pair1.y();
     // pair2.y();   //  pair1和pair2没有y方法
        pair3.y();
    }
}