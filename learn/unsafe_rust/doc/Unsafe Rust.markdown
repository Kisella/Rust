# Unsafe Rust
Rust整体被分为 Safe 和 Unsafe 两部分。Safe Rust 提供高度安全的编程环境，而Unsafe Rust 则用于处理本质上不安全的操作。
在Unsafe Rust 中，程序暴露在不安全的环境下，Rust提供了部分工具来保证安全，但无法避免用户本身进行不安全行为。
## 1 `unsafe` 关键字
使用`unsafe`关键字来使用Unsafe Rust
- `unsafe` 关键字可标记函数、方法、Trait
```rust
unsafe fn unsafe_fn() {}
unsafe trait UnsafeTrait {}
```
&nbsp;
- `unsafe {}`称为**Unsafe Block**, 用于执行非安全操作
```rust
fn main() {
    unsafe {
        unsafe_fn();
    }
}
```
&nbsp;
Unsafe Rust 是 Safe Rust 的超集，所有 safe 的规则（所有权、声明周期、借用等）在 unsafe 中依然生效。
&nbsp;
## 2 Unsafe Rust 的五种能力
在 Unsafe Rust 中，可以进行以下五种非安全操作：
1. 解引用原生指针
2. 调用`unsafe`的函数和方法
3. 修改可变静态变量
4. 实现 `unsafe trait`
5. 读取联合体

这些操作必须在 `unsafe {}` 或标记为`unsafe`的函数中使用
&nbsp;
## 3 原生指针
### 3.1 创建、解引用原生指针
原生指针是 Rust 中最底层的内存访问工具，唯一能彻底突破借用检查的类型。原生指针不遵守借用规则，读写时必须放到`unsafe{}`内
- `*const T`, 指向不可变数据的原生指针
- `*mut T`, 指向可变数据的原生指针
- 使用`&raw`关键字创建原生指针
```rust
fn main() {
    let mut value: i32 = 10;
    let r1: *const i32 = &value as *const i32;
    let r2: *mut i32 = &mut value as *mut i32;  // before Rust 2024
    
    let r1: *const i32 = &raw const value;
    let r2: *mut i32 = &raw mut value;         // ✅ after Rust 2024

    unsafe {
        println!("r1: {}", *r1);
        *r2 += 5;
        assert_eq!(value, 15);
    }
}
```
&nbsp;
### 3.2 原生指针特点
1. 可以是任意地址，空指针，垂悬指针，可能指向非法内存
2. 不具有RAII机制，需要手动管理内存
3. 没有生命周期，不进行借用检查
4. 不保证线程安全，需要程序员自己处理并发问题
&nbsp;
### 3.3 原生指针的常用方法
- `as_ptr()` / `as_mut_ptr()` 将借用转换为原生指针
- `std::ptr::null` 创建空指针
- `is_null()` 判断指针是否为空
- `offset()` / `add()` 进行指针偏移
&nbsp;
## 4 可变静态变量
可变静态变量在多线程场景下会出现数据竞争，因此必须在`unsafe`中访问
- 使用`static mut`声明全局可变静态变量
- 声明全局静态变量时必须显式指明类型
```rust
static mut NUM: i32 = 10;

fn main() {
    unsafe {
        NUM += 5;
    }
}
```
- Rust 不允许创建`static`变量的借用，只允许创建原生指针
```rust
static mut NUM: i32 = 10;

fn main() {
    unsafe {
        NUM += 5;
        // println!("NUM: {}", NUM);  // ❌ 不允许创建引用
        let p1 = &raw const NUM;      // ✅ 创建原始指针
        println!("NUM: {}", *p1);     // 通过原生指针访问值
    }
}
```
&nbsp;
## 5 Union 联合体
联合体允许多个类型的数据共享同一块内存空间，但不提供任何字段的检查，可能导致未定义行为
```rust
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32, 
}

fn main() {
    let u = MyUnion { f: 3.14 };
    
    unsafe {
        println!("i = {}", u.i);    //  ❌ 用错误的类型解释数据
        println!("f = {}", u.f);
    }
}
```
&nbsp;
## 6 `unsafe`方法
标记为`unsafe`的函数并不一定内部不安全，而是调用者必须自己确保不会发生错误
### 6.1 场景1：内部使用`unsafe`操作
```rust
static mut NUM: i32 = 10;
unsafe fn change_static_num() {
    NUM += 1;
}
```
### 6.2 场景2：约束调用者
```rust
// 不检查字节是否符合 UTF-8, 用户必须自己保证
pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
    String { vec: bytes }
}

fn main() {
    unsafe {
        let s = String::from_utf8_unchecked("hello".into());
    }
}
```
&nbsp;
## 7 `unsafe trait`
当 Trait 的实现正确性决定了程序的安全性，但编译器无法自动推断时，需要标记为`unsafe trait`
- 实现`unsafe trait`需使用`unsafe impl`
```rust
unsafe trait Divide {
    fn div(&self, other: Self) -> Self;
}

unsafe impl Divide for i32 {
    fn div(&self, other: Self) -> Self {
        *self / other   // ❌ 实现者未考虑除零错误
    }
}

fn main() {
    // 调用unsafe trait的普通方法无需unsafe块
    10.div(0);  // 除零导致panic, 责任在与实现者
}
```
Trait的`unsafe`, 主要用于提醒实现者而非调用者，而调用者调用一个函数是否要放在 `unsafe {}`中进行调用只取决于函数本身是不是`unsafe`, 而不取决于Trait是否是`unsafe`
```rust
unsafe trait Divide {
    fn div(&self, other: Self) -> Result<Self, &'static str> where Self: Sized;
    unsafe fn div_uncheck(&self, other: Self) -> Self;
}

unsafe impl Divide for i32 {
    fn div(&self, other: Self) -> Result<Self, &'static str> {
        if other == 0 {
            Err("Division by zero!")
        } else {
            Ok(*self / other)
        }
    }


    unsafe fn div_uncheck(&self, other: Self) -> Self {
        *self / other   // 调用者自己注意
    }
}

fn main() {
    unsafe {
        10.div_uncheck(10);
    }
}
```
&nbsp;
- 未标记为`unsafe`的Trait, 也可以声明`unsafe`方法
```rust
trait SafeTrait {
    fn safe_method(&self);
    unsafe fn unsafe_method(&self);
}

impl SafeTrait for i32 {
    fn safe_method(&self) {
        println!("Safe method called on {}", self);
    }
    
    unsafe fn unsafe_method(&self) {
        println!("Unsafe method called on {}", self);
    }
}
```
&nbsp;
当`unsafe`放在不同位置时，背后代表的是这一块代码的责任由谁来承担。当`unsafe`放在函数前面的时候，它表示应该由函数的调用者来承担责任，而当`unsafe`放在 Trait 前面时表示由Trait的实现者来承担这个责任
|`unsafe` 位置|含义|责任主体|
|---|---|---|
|`unsafe {}`|不安全操作| 实现者 |
|`unsafe fn `|不安全的函数或方法，需要自己检查参数合法性|调用者|
|`unsafe trait`|实现`trait`的正确性决定程序的安全性|实现者|