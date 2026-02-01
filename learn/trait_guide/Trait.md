# Trait
在 Rust 中，trait 是定义共享行为的核心抽象机制，类似于面向对象语言中的接口。trait通常可以翻译为**特质**，下文中部分地方使用特质指代trait。

## 1. trait定义
使用**trait**关键字定义一个特质，特质中可定义一个或多个函数签名。trait中定义的方法可以只写函数签名，也可以写出其实现(默认实现)
```rust
// 定义特质Summary
trait Summary {
    fn summarize(&self) -> String; // 定义了一个函数签名
}
```
## 2. trait实现
使用"impl trait名 for 类型名"为某个类型实现某个trait。实现trait时，必须为trait中所有没有默认实现的方法定义其实现。

```rust
// 自定义类型NewsArticle
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// 为类型NewsArticle实现Summary特质
impl Summary for NewsArticle {
    // 为没有默认实现的方法summarize定义其实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## 3. 默认实现与覆盖实现
在trait定义中存在函数体的方法的函数体称作方法的默认实现。当为某个类型实现trait时，我们可以保留或者覆盖哪些默认实现。

```rust
// 定义了HelloWorld特质，存在一个带有默认实现的方法print_hello_world
trait HelloWorld {
    fn print_hello_world(&self) {
    println!("Hello world!");
    } 
}

// 为i32和&str类型实现特质HelloWorld, i32使用默认实现，&str使用覆盖实现
impl HelloWorld for i32 {}
impl HelloWorld for &str {
    fn print_hello_world(&self) {
        println!("Hello world! {self}");
    }
}
```

trait定义中那些没有默认实现的方法可以被trait中的其他方法调用
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // 该方法在定义时只有签名，但仍可被summarize的默认实现调用
    fn summarize_author(&self) -> String;
}
```

## 4. Trait Bound
泛型参数T作为一个类型占位符通常可以用来指代任何类型，但不是所有类型都实现了(某个)trait，所以泛型类型T的实例原本是不能调用任何trait方法的。但如果为泛型T加上"是否实现了某个trait"作为约束，则那些带有这种约束的实例就可以放心调用trait方法了。
这种以实现了某些trait作为泛型参数T约束的约束条件，称作泛型参数的Trait Bound(特质约束)，带有Trait Bound的泛型参数可以调用指定trait的方法。

特质约束通常有三种形式，这三种形式效果等价。
```rust
trait Summary {}
trait HelloWorld {}

// 特质约束定义在泛型参数后
fn api_1<T: Summary + HelloWorld>(item: &T) {}

// 特质约束定义在入参后
fn api_2(item: &(impl Summary + HelloWorld)) {}

// 特质约束定义在where子句
fn api_3<T>(item: &T)
where
    T: Summary + HelloWorld,
{
}
```
对于多种泛型参数具有不同特质约束的复杂情况，通常使用where子句的形式
```rust
fn api_4<T, U>(item1: &T, item2: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

## 5. blanket 实现
可以为实现了某个特质的类型连带实现另外一个trait，称为blanket实现。例如在标准库中为实现了Display特质的类型也实现了ToString特性。
```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

## 6. 泛型类型的trait差异化实现
对于带有泛型参数的类型(结构体或枚举), 可以根据不同的trait bound进行差异化实现。
```rust
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
```
## 7. 关联类型
在Rust中，关联类型（Associated Types） 是 trait 定义中的一种特殊类型占位符，它允许 trait 的方法签名中使用这些类型而不需要提前指定具体类型。
```rust
trait Processer {
    type Input: Display;
    type Output: Display;
    fn process(&self, input: Self::Input) -> Self::Output;
}

struct TextProcessor;
struct NumberProcessor;

impl Processer for TextProcessor {
    type Input = String;
    type Output = usize;
    fn process(&self, input: Self::Input) -> Self::Output {
        input.len()
    }
}

impl Processer for NumberProcessor {
    type Input = i32;
    type Output = f64;
    fn process(&self, input: Self::Input) -> Self::Output {
        (input * 2).into()
    }
    
}
```

## 9.静态派发&动态派发
### 1. 静态派发
静态派发是 Rust 在编译期间确定具体调用哪个方法实现的过程。它通过泛型和单态化（monomorphization）实现。静态派发时编译器为每个具体类型生成专用版本的方法，方法调用直接指向具体实现，无运行时开销。

### 2. 动态派发
动态派发在运行时确定具体调用哪个方法实现。使用Trait 对象和通过虚表（vtable）实现。每次方法调用需额外指针跳转，存在运行时开销。相比静态派发，动态派发只需一份代码，无二进制文件膨胀问题，且支持运行时多态和异构集合，具备更强的灵活性。
```rust
trait Widget {
    fn render(&self);
}
struct Button {
    text: String,
}
struct CheckBox {
    checked: bool,
}
impl Widget for Button {
    fn render(&self) {
        println!("Rendering buttom: {}", self.text);
    }
}
impl Widget for CheckBox {
    fn render(&self) {
        println!("CheckBox: {}", self.checked);
    }
}

// 使用动态派发，在容器中储存不同的类型对象
fn build_ui() -> Vec<Box<dyn Widget>> {
    vec![
        Box::new(Button { text: "OK".into() }),
        Box::new(CheckBox { checked: true }),
    ]
}

fn main() {
    let ui = build_ui();
    for widget in ui {
        widget.render();
    }
}
```

### 3. 对象安全
一个 trait 可用于动态派发（dyn Trait）必须满足“对象安全”
1. 方法不能返回Self（运行时无法确定具体类型大小）
2. 方法不能有泛型参数（无法在运行时单态化）
3. Trait 不能包含关联常量（虚表无法存储常量值）
4. 所有方法必须是可派发的，不能存在静态方法（无 self 参数）

## 10. 多态
与面向对象语言的区别：
- 实现位置：
Rust：在类型外部实现 (impl Animal for Dog)
Java：在类内部实现 (class Dog implements Animal)
- 多态机制：
Rust：可选择静态或动态分发
Java：始终使用动态分发（虚方法表）
- 数据与行为分离：
Rust：trait 只定义行为，不包含数据
Java：接口可包含常量字段
- 类型系统集成：
Rust：trait 可约束泛型 (T: Animal)
Java：接口不能直接约束泛型类型参数

```rust
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

fn main() {
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
```