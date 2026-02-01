mod polymorphism {
    use std::any::Any;

    // 定义一个表示动物的trait
    // 通过Any实现具体类型的向下转型，以支持类型查询
    trait Animal: Any {
        fn speak(&self) -> String;
        fn sleep(&self) -> String {
            "Zzz...".to_string()
        }
    }

    struct Dog {
        name: String,
    }
    struct Cat {
        name: String,
        lives: u8,
    }
    struct Bird;

    impl Animal for Dog {
        fn speak(&self) -> String {
            format!("{} says: Woof!", self.name)
        }
        fn sleep(&self) -> String {
            format!("{} is dreaming of bones...", self.name)
        }
    }
    impl Animal for Cat {
        fn speak(&self) -> String {
            format!("{} says Meow! ({} lives left)", self.name, self.lives)
        }
    }
    impl Animal for Bird {
        fn speak(&self) -> String {
            "Chirp chirp!".to_string()
        }
    }

    // 静态派发（编译时多态），零成本抽象
    fn animal_speak_static<T: Animal>(animal: &T) -> String {
        animal.speak()
    }

    // 动态派发（运行时多态）， 使用trait对象
    fn animal_speak_dynamic(animal: &dyn Animal) -> String {
        animal.speak()
    }

    #[test]
    fn test() {
        let dog = Dog {
            name: "Buddy".to_string(),
        };
        let cat = Cat {
            name: "Whiskers".to_string(),
            lives: 7,
        };
        let bird = Bird;

        // ===== 静态分发示例 =====
        println!("\n=== 静态分发 (编译时多态) ===");
        println!("{}", animal_speak_static(&dog)); // 调用 Dog 实现
        println!("{}", animal_speak_static(&cat)); // 调用 Cat 实现
        println!("{}", animal_speak_static(&bird)); // 调用 Bird 实现

        // ===== 动态分发示例 =====
        println!("\n=== 动态分发 (运行时多态) ===");
        let animals: Vec<&dyn Animal> = vec![&dog, &cat, &bird];
        for animal in animals {
            println!("{}", animal_speak_dynamic(animal));
            println!("{}", animal.sleep());

            // 动态分发允许运行时类型识别
            if let Some(cat) = (animal as &dyn Any).downcast_ref::<Cat>() {
                println!("Special cat operation! Lives: {}", cat.lives);
            }
        }
    }
}
