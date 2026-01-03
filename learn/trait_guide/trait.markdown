# trait
---
Trait 的含义是特征，表示类型所具有的能力。Trait 是 Rust 最强大的特性之一，既是类型系统的支柱，也是泛型约束的核心机制。
> `i32` 可以做加法，因为实现了 `add` Trait
> `char` 不能做加法，因为没有实现 `add` Trait

Trait 具有四大功能:
- 抽象接口
- 泛型约束
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
## 1.2 Trait 泛型
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
## 1.3 关联类型
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
