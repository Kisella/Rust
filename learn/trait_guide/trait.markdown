# trait
---
Trait 的含义是特征，表示类型所具有的能力。Trait 是 Rust 最强大的特性之一，既是类型系统的支柱，也是泛型约束的核心机制。
> `i32` 可以做加法，因为实现了 `add` Trait
> `char` 不能做加法，因为没有实现 `add` Trait

Trait 具有四大功能:
- 抽象接口
- 泛型特征约束
- 抽象类型
- 运算符重载

## 1 抽象接口
Trait 可以定义一些共享行为，它类似于其他语言中的接口（interface），但更加强大。 
- 使用 `trait` 关键字定义一个特征
- 特征中可以定义方法，并支持默认实现
- 特质之间可以继承
- 使用 `impl` 关键字为类型实现特征方法
- 同一个特质可以被多个类型实现，但不能被一个类型实现多次

### 1.1 定义和实现 Trait
Trait定义了一组可以被共享的行为，只要类型实现了 Trait，该类型就能使用这组行为。
- 使用 `trait` 关键字定义一个特征
- 特征定义方法只需写函数签名，用`;`代替函数体
- 方法支持使用 `self` 及其变体作为第一个参数

```rust
trait t_name {
    fn func_1(args...) -> ret;
    fn func_2(&self, args...) -> ret;
}
```
&nbsp;
- 使用 `impl` 关键字为类型实现特征方法
- 必须实现所有未提供默认实现的方法
- 提供默认实现的方法支持覆盖实现
```rust
impl t_name for type {
    fn func_1(args...) -> ret {
        // ...
    }

    fn func_2(args...) -> ret {
        // ...
    }
}
```
&nbsp;
示例：
```rust
struct Person {
    name: String,
    age: u8,
    can_swim: bool,
}

struct Duck {
    color: String,
}

impl Swim for Person {
    fn swim(&self) {
        match self.can_swim {
            true => println!("I've learned to swim."),
            false => println!("I can't swim.")
        }
    }
}

impl Swim for Duck {
    fn swim(&self) {
        println!("I was born to swim.")
    }
}

fn main() {
    Person {name: String::from("zhangsan"), age: 28, can_swim: true}.swim();
    Duck {color: String::from("yellow")}.swim();
}
```
```rust
trait Swim{
    fn swim(&self) {
        println!("I was born to swim.")
    }
}

impl Swim for Duck {}  // 使用默认实现

impl Swim for Person {
    fn swim(&self) {   // 覆盖实现
        match self.can_swim {
            true => println!("I've learned to swim."),
            false => println!("I can't swim.")
        }
    }
}
```
&nbsp;
### 1.2 Trait 泛型
- 在trait名后使用`<>`声明泛型参数，可定义Trait泛型
- Trait泛型经过单态化后才成为具体的trait
```rust
trait MySwap<T> {
    fn my_swap(self, other: T) -> (T, Self);
}
impl<T> MySwap<T> for i32 {
    fn my_swap(self, other: T) -> (T, Self) {
        (other, self)
    }
}

// Myswap<i32> 与 MySwap<i64>是两个不同的类型
```
&nbsp;
### 1.3 关联类型
在很多情况下，对于同一个trait泛型，不同的参数类型往往需要不同的返回类型。我们很容易想到可以在返回值中使用泛型来实现这一需求：
```rust
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

fn main() {
    let ret: i32 = 1.my_add(10);
    let ret: i64 = 1.my_add(10_i64);
}
```
但在返回值中使用泛型参数存在一个问题。当参数类型确定时，返回类型应该也是确定的，但泛型参数却允许返回不定类型。这显然不符合类型安全的要求。
```rust
// i32 + i32 -> i32
impl MyAdd<i32, i32> for i32 {
    //  i32 + i32 -> i32
    fn my_add(self, other: i32) -> i32 {
        self + other
    }
}

// i32 + i32 -> i64 !
impl MyAdd<i32, i64> for i32 {
    // i32 + i64 -> i64
    fn my_add(self, other: i32) -> i64 {
        (self + other) as i64
    }
}

fn main() {
    // i32 + i32 可能返回i32或i64, 不得不显式指明返回类型
    let ret: i32 = 1.my_add(10);  
}
```
- 我们希望参数类型确定时，对应的返回类型也是确定的。此时我们可以引入**关联类型**
- 使用`type`关键字在Trait定义中声明关联类型
- 使用`type`关键字在Trait实现中指定关联类型
- 通过`Self::Name`在Trait的定义和实现中使用关联类型
```rust
trait MyAdd<I> {
    type O;
    fn my_add(self, other: I) -> Self::O;
}
//  i32 + i32 -> i32
impl MyAdd<i32> for i32 {
    type O = i32;
    fn my_add(self, other: i32) -> Self::O {
        self + other
    }
}
//  i32 + i64 -> i64
impl MyAdd<i64> for i32 {
    type O = i64;
    fn my_add(self, other: i64) -> Self::O {
        self as i64 + other
    }
}
```
> 泛型 VS 关联类型
> - 关联类型在实现时确定类型，泛型在调用时确定类型
> - 其他泛型确定类型时，某个泛型也得到一个确定类型 -> 使用关联类型
> - 用户自己决定调用时得到的类型 -> 使用泛型
&nbsp;
### 1.4 关联常量
- Trait定义中支持使用`const`关键字声明关联常量
- 与关联类型类似，关联常量在定义阶段声明，在实现阶段确定值
```rust
trait Limit {
    const MAX: u32;
}

struct Counter;
impl Limit for Counter {
    const MAX: u32 = 100;
}
```
- 关联常量支持默认值
```rust
trait Limit {
    const MAX: u32 = 1000;
}
```
&nbsp;
### 1.5 SupperTrait
- 在trait名后使用`:`添加trait依赖
- 如何某类型要实现该trait, 必须先实现了该trait依赖的所有trait
- 特征约束的一种特例
```rust
trait t_name: t1 + t2 ... {
    // ...
}
```
```rust
trait Introduce: Display {
    fn introduce(&self) {
        println!("我是{}", self);
    }
}

struct Cat { name: String}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "猫咪，{}", self.name)
    }
}
impl Introduce for Cat {}

fn main() {
    let cat = Cat{ name: String::from("柴郡")};
    cat.introduce();
}
```
&nbsp;
### 1.6 完全限定语法
当存在多个 Trait 的方法名冲突时，需要使用完全限定语法来避免歧义。方法调用的推导如下：
- 先在类型的固有方法中查找对应方法
- 固有方法中没找到，则在该类型实现的所有trait中查找
- 如何存在多个trait方法候选，报错提示需要显式指定
&nbsp;
- 完全限定语法：`<Type as Trait>::method(args...)`
```rust
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying the plane!");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying with magic!");
    }
}

fn main() {
    let h = Human;
    //  h.fly();            //  存在歧义，不能直接用点运算符调用
    Pilot::fly(&h);     //  trait::method 调用
    <Human as Wizard>::fly(&h);  // 完全限定语法调用
}
```
- 当 Trait 方法与类型的固有方法运算符冲突时，点运算符优先调用固有方法，这时需要使用`trait::method(args..)`或完全限定语法来调用对应的trait方法。
```rust
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying with magic!");
    }
}
impl Human {
    fn fly(&self) {
        println!("human flapping arms... not very effective.");
    }
}

fn main() {
    let h = Human;
    h.fly();            //  调用固有方法
    Wizard::fly(&h);     //  trait::method 调用
    <Human as Wizard>::fly(&h);  // 完全限定语法调用，推荐
}
```
&nbsp;
### 1.7 孤儿规则
如何某类型要实现某个Trait, 那么该Trait和要实现该Trait的类型至少有一个要在当前Crate中定义。
- 外部类型不支持实现外部Trait
```rust
impl std::fmt::Display for Vec<i32> {
    // 拒绝，编译错误
}
```
&nbsp;
- 外部类型支持实现本地Trait
```rust
trait MyTrait {}
impl MyTrait for Vec<i32> {
    // 合法
}
```
&nbsp;
- 本地类型支持实现外部Trait
```rust
struct MyStruct;
impl std::fmt::Display for MyStruct {
    // 合法
}
```
&nbsp;
### 1.8 NewType
想为外部类型实现外部Trait时，使用 NewType 模式绕过孤儿规则
- 语法: `struct NewType(Type)`
```rust
use std::fmt::Display;

struct MyString(String);    //  NewType, 包裹外部类型

impl Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyString says: {}", self.0)
    }
}

fn main() {
    let s = MyString(String::from("hello"));
    println!("{s}");
}
```
&nbsp;
## 2 泛型特征约束
### 2.1 约束即能力
考虑这样一个场景，我们比较两个值的大小，然后他们之中的较大者。我们直接在参数中使用泛型，Rust拒绝了这个函数定义。原因很简单，一个不做任何约束的泛型允许单态化为任意的类型，但不是任意类型都支持比较大小。
```rust
fn get_max<T> (a: T, b: T) -> T {
    if a > b { a } else { b } //  编译失败，不是任何类型都支持比较大小。
}
```
我们为泛型参数T加上PartialOrd的**特征约束(trait bounds)**，函数只接受实现了PartialOrd的类型，这样就能确保函数内部进行比较操作时是安全的。Rust接受了这个函数定义。
```rust
// 约束：只有实现了PartialOrd的类型才能使用
fn get_max<T: PartialOrd> (a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 这些类型被"排除"在外：
struct MyType;  // 没有实现PartialOrd，无法使用compare函数
```
实际上，为某一类型实现某个Trait，这一行为可以认为是赋予这个类型某个方面的能力。与之相对应，泛型的特征约束可以认为是赋予泛型参数某个方面的能力，使泛型参数能够安全的进行特定操作。
特征约束在Rust中起到了双重作用：
**约束**：它限制了可以被用来实例化泛型参数的类型。只有那些实现了指定trait的类型才能使用，从而保证了类型的安全性。
**赋能**：它赋予了泛型参数某种能力，使得在泛型函数或结构体中，我们可以基于这些能力来编写代码。
特征约束既是一种约束（对类型的限制），也是一种赋能（允许泛型代码使用这些能力）。两者是同一事物的两面。通过约束类型必须实现某些trait，我们赋予了这些类型在泛型上下文中的能力。
&nbsp;
### 2.2 语法
- `T: Trait_1 + Trait_2 + ...`
- 在泛型声明中使用`:`对泛型进行约束，只有实现了某些Trait的类型才能传入
```rust
// 函数声明中使用特征约束，限制参数传入，同时赋予泛型参数能力
fn get_max<T: PartialOrd> (a: T, b: T) -> T {
    if a > b { a } else { b }
}
```
- `impl`块中使用特征约束，为一系列类型提供通用实现
```rust
struct Point<T> {
    x: T,
    y: T,
}

// impl块中使用特征约束，为一系列类型提供通用实现，同时赋予泛型参数能力
impl<T: std::fmt::Display> Point<T> {
    fn show(&self) {
        println!("x = {}, y = {}", self.x,  self.y);
    }
}
```
&nbsp;
### 2.3 限制关联类型
- 特征约束支持限制关联类型
- `T: Trait<Type = T>`
```rust
impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    fn add(&self, other: &Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```
&nbsp;
### 2.4 类型中的特征约束
- 自定义类型支持特征约束
- 在类型定义时进行约束后，impl时仍需写出约束；此用法较少，一般在impl层面进行约束
```rust
struct Holder<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> Holder<T> {
    pub fn print_value(&self) {
        println!("value is {}", self.value);
    }
}
```
&nbsp;
### 2.5 泛型Trait中的特征约束
- 泛型Trait支持特征约束
```rust
trait Summary<T: std::fmt::Display> {
    fn summarize(&self, item: T) -> String;
}

struct News;

impl Summary<String> for News {
    fn summarize(&self, item: String) -> String {
        format!("新闻摘要：{item}")
    }
}
```
&nbsp;
### 2.6 关联类型中的特征约束
-  关联类型支持特征约束
-  后续impl时，关联的类型必须实现了特定的Trait
```rust
trait Container {
    type Item: Display;
}
impl Container for News {
    type Item = String;
}
```
NewsNews
&nbsp;
### 2.7 Blanket Implementation
- 语法`impl<T: Trait_1 + Trait_2 + ...> Trait for Type<T>`
- 为实现了某些Trait的所有类型实现一个Trait
- 为一系列类型提供通用实现
```rust
use std::fmt::Display;
trait Introduce: Display {
    fn introduce(&self);
}

struct Cat { name: String}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "猫咪，{}", self.name)
    }
}
// 为所有实现了Display的类型提供Introduce的通用实现
impl<T: Display> Introduce for T {
    fn introduce(&self) {
        println!("我是{}", self);
    }
}

fn main() {
    let cat = Cat{ name: String::from("柴郡")};
    cat.introduce();
}
```
&nbsp;
### 2.8 Where 子句
当特征约束复杂时，使用`where`子句提升可读行
- 使用`where`关键字定义子句
- 子句内可以对所有声明过的泛型参数指明特征约束
- 每个泛型用`:`表示约束
- 多个泛型约束之间用`,`隔开
```rust
// 定义类型
struct MyType<T, U>
where
    T: Trait_1 + Trait_2 ... ,
    U: Trait_3 + Trait_4 ... ,
{}

// 定义方法
impl<T, U> Mytype<T, U>
where
    T: Trait_1 + Trait_2 ... ,
    U: Trait_3 + Trait_4 ... ,
{}

// 定义函数
fn func<T, U>() -> T
where
    T: Trait_1 + Trait_2 ... ,
    U: Trait_3 + Trait_4 ... ,
{}
```
```rust
// 定义泛型
trait MyTrait<T, U>
where
    T: Trait_1 + Trait_2 ... ,
    U: Trait_3 + Trait_4 ... ,
    Self: Trait_5 + Trait_6 ... ,   //  SupperTrait约束（不推荐）
{
    // 关联类型（不推荐）
    type Item where Self::Item: Trait_7; 
}
```
- SupperTrait和关联类型中使用where子句不能增加可读性，不推荐使用，直接使用`:`约束即可
&nbsp;
### 2.9 嵌套约束
有时不仅要约束泛型T，还要约束T内部包含的类型（关联类型）
```rust
trait Team{
    type Leader;
}

trait Speak {
    fn say_hello(&self);
}

fn interview<T>(team: T, leader: T::Leader)
where
    T: Team,
    T::Leader: Speak,
{
    leader.say_hello();
}
```
&nbsp;
### 2.10 关联类型中的完全限定语法
当一个类型实现了多个Trait, 且这些Trait存在同名关联类型时，需要完全限定
```rust
trait Pilot {
    type Badge; //  飞行员徽章
}
trait Engineer {
    type Badge; //  工程师徽章
}

fn check_badge<T: Pilot + Engineer>() {
    // let my_badge: T::Badge; //  出现歧义，编译器不知道是那个Badge
    let pilot_badge: <T as Pilot>::Badge;
    let engineer_badge: <T as Engineer>::Badge;
}
```
- 嵌套约束的最严谨写法，使用完全限定（推荐）
```rust
fn interview<T>(team: T, leader: <T as Team>::Leader)
where
    T: Team,
    <T as Team>::Leader: Speak,
{
    leader.say_hello();
}
```
