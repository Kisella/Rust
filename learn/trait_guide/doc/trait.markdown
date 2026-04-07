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
- 特征之间可以继承
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
&nbsp;
- `var: impl Trait_1 + Trait_2`
- 在参数或返回值中使用`impl`关键字，是泛型特征约束的一种简便写法，不用显式地写出泛型
- 泛型可以根据Turbofish语法显式指定类型，而使用`impl`只能编译器自行推导类型
```rust
fn func(arg: impl Trait_1 + Trait_2);
```
等效与以下代码:
```rust
fn func<T: Trait_1 + Trait_2>(arg: T);
```
- 在编译器不能准确推断出类型时，`impl`语法不适用
```rust
// 显式地显式泛型，编译通过
fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 编译失败。由于没有显式指定泛型，编译器只能默认认为a和b是不同类型，进而导致编译器认为函数返回了多个类型
fn get_max(a: impl PartialOrd, b: impl PartialOrd) -> impl PartialOrd {
    if a > b { a } else { b }
}

// 也可以`impl`语法和泛型参数组合使用
fn get_max<T: PartialOrd>(a: T, b: T) -> impl PartialOrd {
    if a > b { a } else { b }
}
```
&nbsp;
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
&nbsp;
### 2.7 Blanket Implementation
- 语法`impl<T: Trait_1 + Trait_2 + ...> Trait for Type<T>`
- 为实现了某些Trait的所有类型实现一个Trait
- `impl`带上泛型，表明为一系列类型提供实现
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
&nbsp;
## 3 抽象类型
在面向对象语言中，父类型可以接收子类型为参数，从而实现多态。例如`Duck`、`Person`、`Dog`都可以游泳，都有`swim`方法，都继承`Swimmer`类，函数参数接收`Swimmer`指针，实际可传入任意子类型。
Rust不是原生的面向对象语言，但提供类似的机制，即特征对象(**Trait Object**)
## 3.1 Trait Object
把实现了某个Trait的所有类型看做一个集合，这个集合又可以看做一个新的抽象的类型，称作特征对象
- 函数参数类型声明中使用`&dyn Trait`声明接收一个特征对象
- 特征对象必须基于借用、指针、智能指针来间接使用
```rust
// 定义对象
struct Duck;
struct Person;
struct Dog;

// 定义共同行为
trait Swimmer {
    fn swim(&self);
}

// Duck, Person, Dog都实现了Swimmer特质，所有都属于抽象类型&dyn Swimmer
impl Swimmer for Duck {
    fn swim(&self) {
        println!("Duck paddles through the pond.");
    }
}
impl Swimmer for Person {
    fn swim(&self) {
        println!("Person swims freestyle.");
    }
}
impl Swimmer for Dog {
    fn swim(&self) {
        println!("Dog does the doggy paddle.");
    }
}

// 定义函数，接受一个Trait Object作为参数参数
fn make_swim(swimmer: &dyn Swimmer) {
    swimmer.swim();
}

fn main() {
    let duck = Duck;
    let person = Person;
    let dog = Dog;

    make_swim(&duck);
    make_swim(&person);
    make_swim(&dog);
}
```
&nbsp;
## 3.2 动态大小类型
对于一个Trait来说，它可以被多个类型实现，而实现它的各种类型之间大小可能是不同的。故特征对象`dyn Trait`是一个动态大小类型(DST)。动态大小类型不能在函数中直接使用，因为函数的栈帧大小需要在编译期就要确定，因此需要一个指针来间接操作动态大小类型。
- 实现Trait的类型的大小是不确定的
- 特征对象作为一个大小不确定的抽象类型只能通过指针作为函数参数
&nbsp;
## 3.3 虚表
指向特征对象的指针`&dyn Trait`是一个**胖指针**，包含`data`指针和`v-ptr`指针两部分
- `data`指针指向`Self`, 即实现了该Trait的具体类型
- `v-ptr`指向`v-table`, 即**虚表**，虚表中存储了类型`Self`的信息，包括size、对其方式、trait方法等
- 同一个特征对象，虚表内部的结构是确定的，所以无论`v-ptr`指向哪个类型的虚表，都可以正确地找到对应实现的Trait方法

通过`data`和`v-ptr`这两个指针，特征对象在调用方法时就可以方便地获得函数地址和传递参数
&nbsp;
## 3.4 动态分发
在调用特征对象对应Trait方法时，首先将胖指针`&dyn Trait`拆成`data`和`v-ptr`, 随后依据固定的虚表布局找到对应偏移量的函数指针，直接通过这个函数指针将参数传递就完成了调用。
```rust
    let (data, vptr) = demo // 拆胖指针
    let f = vptr[4]         // 偏移量 4 是 func2 的槽位
    f(data);                // 调用
```
上述特征对象调用Trait方法的过程发生在**运行时**，只有在运行时才能确定要调用的函数的地址，这个过程需要查表、传参，因此运行时效率会有所降低，但灵活性会有所提高
这种运行时通过虚表和偏移量查找函数并调用的过程，称为动态分发(Dynamic Dispatch)。因为“动态”是dynamic，所以特征对象使用了`dyn`关键字
- 一个类型实现了某个trait时就会生成对应的虚表
- 同一个类型的所有实例共用同一张虚表
- 运行时特征对象通过查虚表，即时获得函数的地址
## 3.5 多Trait动态分发
实际开发中，时常会遇到需要对实现了多个Trait的类型进行动态分发的场景
- 某类型同时实现了多个Trait
- 在运行时通过特征对象来调用这些trait方法
正确做法是用一个新的Trait去依赖所有需要的trait, 将其打包成一个组合trait, 然后再对这个组合trait作动态分发：
- 定义一个新的Trait, `NewTrait`
- 需要依赖的Trait作为`NewTrait`的SuperTrait
- 使用`&dyn NewTrait`作为Trait对象
```rust
// 定义基础Trait
use std::fmt::Debug;

trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

// 定义类型，并实现基础trait
#[derive(Debug)]
struct S;
impl Foo for S {
    fn foo(&self) {
        println!("foo");
    }
}

impl Bar for S {
    fn bar(&self) {
        println!("bar");
    }
}

// 定义组合trait，依赖基础trait
trait FooBarDebug: Foo + Bar + Debug {}

// 使用Blanket实现，为所有实现了Foo、Bar和Debug的类型自动实现FooBarDebug
impl<T> FooBarDebug for T where T: Foo + Bar + Debug {}

// 定义函数，接受一个组合特征对象作为参数
fn call_all(x: &dyn FooBarDebug) {
    x.foo();
    x.bar();
    println!("{:?}", x);
}

fn main() {
    let s: S = S;
    call_all(&s);
}
```
## 3.6 动态分发限制
一个Trait能够成为特征对象并进行动态分发，它就被称为`dyn-compatible`, `dyn-compatible`必须满足以下五个条件：
- SurperTrait都是`dyn-compatible`
- SurperTrait中不能有`Sized`
- 不能有关联常量
- 不能有带泛型的关联类型
- 所有函数必须可动态分发或显式标记为不可调用
### 3.6.1 SurperTrait中不能有`Sized`
`Sized`是一个原生的Trait, 他表示类型的大小是固定的。而特征对象是一个动态大小类型，没有确定的大小，这与`Sized`约束存在根本性冲突。
-  绝大部分Trait默认依赖`?Sized`, 表示不限制类型的大小
-  当一个Trait受到`Sized`约束，它就不能作为Trait对象
```rust
trait Demo: Sized {} // 不能作为Trait对象
```
&nbsp;
### 3.6.2 不能有关联常量
方法可以通过虚表动态分发到正确的函数指针，但常量不是函数调用，无法通过虚表动态查找一个常量的值。故Trait对象要禁止关联常量
```rust
trait Foo {
    const A: i32;
}
impl Foo for u8  { const A: i32 = 1; }
impl Foo for u16 { const A: i32 = 2; }
// data和v-ptr中都不含有关联常量的信息
// 问题： 如果有一个 Box<dyn Foo>, 那么dyn Foo: A 应该是1还是2?
```
&nbsp;
### 3.6.3 不能有泛型关联类型
关联类型往往在对应方法中使用，用户每指定一个泛型作为关联类型，就会单态化出一个新版的函数，最后导致虚表大小不确定，格式不一致。故Trait对象要禁止泛型关联类型。
&nbsp;
### 3.6.4 可动态分发的函数
并不是Trait内部的方法都会进入虚表，只有可通过特征对象动态分发的方法才会进入虚表，
可动态分发的函数必须满足3个条件：
- 必须使用`self`或可解引用为`self`的类型作为第一个参数（即实例方法）
- 函数签名中不能在第一个参数以外的位置使用`Self`
- 不能有泛型参数
#### 3.6.4.1 第一个参数必须是`self`
动态分发过程中，从虚表拿到函数地址后会将`data`作为第一个参数传入，如果函数第一个参数不是`self`或`self`的指针, 参数传递就会出错
```rust
    let (data, vptr) = demo 
    let f = vptr[4]
    f(data);
```
#### 3.6.4.2 其他位置不能使用`Self`
函数签名中，不能在第一个参数以外的位置使用`Self`、`&Self`等含`Self`的类型
```rust
fn func(&self, other: Self) -> Self
```
```rust
trait Test {
    fn merge(&self, other: &Self);
}

fn fun(t1: &dyn Test, t2: &dyn Test) {
    t1.merge(t2);
}
// 对于func函数来说，它只保证t1和t2是实现了Test的类型，但不能保证t1和t2最终指向了相同的类型
```
特征对象是一个抽象类型，它的类型信息被擦除了，若允许其他位置使用了`Self`的方法的动态调用，不能保证`self`与`other: Self`的类型是一致的。
#### 3.6.4.3 不能有泛型参数
当函数带有泛型参数，经过单态化后可能派生出无数种具体实现
- 问题1：无法预知会单态化出多少个版本，虚表装不下这么多函数地址
- 问题2：不同类型单态化出的版本不同，虚表格式不统一
&nbsp;
#### 3.6.4.4 显式标记为不可调用
如果某个函数违背了可动态分发的规则，必须显式标记为不可调用，否则trait不能动态分发
- 函数签名中添加`where Self: Sized`, 标记方法不可用于trait对象
```rust
trait Demo {
    fn demo(&self);
    fn func(&self, other: Self) -> Self
    where Self: Sized;  //  ✅ 添加 Trait 约束，显式标记为不可调用
    fn func2(num: i32)
    where Self: Sized;  //  ✅ 添加 Trait 约束，显式标记为不可调用 
}

impl Demo for Type1 {
    fn demo(&self) {
        println!("Tpye1 demo");
    }

    fn func(&self, other: Self) -> Self
        where Self: Sized {
        println!("Type1 func");
        other
    }

    fn func2(num: i32)
        where Self: Sized {
        println!("Type1 func2 with num: {}", num);
    }
}

fn process(d: &dyn Demo) {
    d.demo();           // ✅ 可以调用
    // d.func(Type1);  // ❌ 不可调用
    // d.func2(42);    // ❌ 不可调用
}
```
&nbsp;
## 3.7 静态分发 VS 动态分发

|  对比维度  | 静态分发 | 动态分发|
| :------: | :--- | :----- |
|    机制    | 使用 `Trait Bound`（泛型约束）| 使用 `Trait Object`（`dyn Trait`）|
|  语法形式  | `fn foo<T: Trait>(t: T)`<br/>`impl<T: Trait> Struct<T>` | `fn foo(t: &dyn Trait)`<br/>`Box<dyn Trait>`<br/>`&mut dyn Trait` |
|  分发时机  | 编译期 | 运行时 |
|  分发方式  | 单态化（Monomorphization）：<br/>为每种具体类型生成独立函数版本 | 虚表（vtable）查找：<br/>通过指针间接调用 |
|  分发数量  | 编译期确定（取决于实际使用的类型数量）| 运行期确定（可处理任意数量的类型）|
|  性能开销  | 零成本抽象：<br/>- 无运行时开销<br/>- 可内联优化<br/>- 无间接调用 | 有性能损失：<br/>- 虚表查找（1-2次指针解引用）<br/>- 无法内联（阻碍优化）<br/>- 缓存局部性差 |
|  代码膨胀  | 可能导致代码膨胀（每个类型生成一份代码）| 代码紧凑（共享同一份代码）|
|   灵活性   | 较低（类型必须在编译期确定）| 较高（运行时可处理任意实现该trait的类型）|
|  类型大小  | 使用具体类型，大小已知（`Sized`）| Trait对象是DST（动态大小类型），需配合指针使用 |
| 对象安全性 | 不要求（所有方法都可用）| 要求trait是对象安全的：<br/>- 不能有`Self`返回类型（除`Self: Sized`外）<br/>- 不能有泛型方法<br/>- 不能消费`self`（除`Self: Sized`外） |
|  泛型支持  | 支持泛型方法和关联类型 | 不支持泛型方法（会破坏对象安全）|
|  编译时间  | 可能增加编译时间（单态化生成多份代码）| 编译时间较短（只生成一份代码）|
| 二进制大小 | 可能较大（代码膨胀） | 较小（代码共享）|
|  多态类型  | 编译期多态（静态多态）| 运行时多态（动态多态）|
|  调用开销  | 直接调用或内联（最快）| 间接调用（虚表查找）|
|  类型擦除  | 无（保留完整类型信息）| 有（擦除具体类型，只保留trait信息）|

## 3.8 多态


