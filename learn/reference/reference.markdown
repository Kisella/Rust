# 借用
## 0 基本概念
**借用**，在Rust中通常指不转移所有权的使用权，它支持我们可以在不取得变量所有权的情况下对变量进行读取或修改。为避免借用而引起内存安全问题，Rust的借用系统秉持**共享不可变，可变不共享**的基本原则。
在Rust中，**引用**和***借用**的概念是紧密相关的，但又不完全相同：
- 引用(**Reference**), 通常指一种类型，使用`&`或`& mut`符号创建，它是一个指向某个值的指针，但不拥有该值的所有权。例如`let r = &x`,这里的`r`是一个引用。
- 借用(**Borrowing**), 通常指一种行为，指通过引用来临时访问某个值的动作，而不转移其所有权。借用规则由编译器强制执行，确保内存安全。

可以认为，引用是借用的具体实现方式。当创建一个引用时，就是在借用对应的值。

## 1 共享引用
- 使用`&`符号取得变量的共享引用
- 使用`ref`关键字取得变量的共享引用
- 共享引用允许读取数据，但不允许修改数据
```rust
    let s: String = String::from("Hello, world!");
    let r1: &String = &s;
    let ref r2: &String = &s;

    println!("原始字符串：{s}");
    println!("引用 r1: {r1}");
    println!("引用 r2: {r2}");
```
```rust
    let mut v =Box::new(42);
    let r = &v;
    // **r += 1; // 不能通过共享引用修改值
```
&nbsp;
## 2 可变引用
- 使用`& mut`符号取得变量的可变引用
- 使用`ref mut `关键字取得变量的可变引用
- 只有当变量被声明为可变时，才能取得它的可变引用
- 可变引用允许读取或修改数据
- 使用可变引用修改数据时，需使用解引用符号`*`
```rust
    let mut s: String = String::from("Hello");
    let r1: &mut String = &mut s; // 可变引用
    r1.push_str(", world"); // 通过可变引用修改字符串
    let ref mut r2 = s;
    r2.push_str("!");
    println!("修改后的字符串：{s}");
```
```rust
    let v = 42;
    // let r = &mut v; // 变量未声明为可变，不可取得可变引用
    let mut v = v; // 将 v 声明为可变
    let r = &mut v; // 现在可以取得可变引用
    *r += 1; // 通过可变引用修改值
    println!("修改后的值：{}", *r);
```
&nbsp;
- 已经确定引用类型的变量不会自动转化类型
```rust
    let mut x = 0;
    let mut y = 1;
    let mut reference = &x;
    println!("{reference}");
    reference: &i32 = &mut y; //   不推荐，reference是可变的，故可以变化绑定为y的引用，但reference的类型已经被确定为不可变的&i32，故这里的mut是无效的
    println!("{reference}");
    // *reference += 1;   //   reference的类型依然是&i32不可变引用，故不能修改y的值
    let reference: &mut i32 = &mut y; //  推荐，重新声明变量接收y的可变引用
    *reference += 1;
    println!("{reference}");
```
## 3 借用规则
Rust的官方文档中的借用规则如下:
- 规则1：在同一时间内，一个变量只允许存在多个共享引用，或一个可变引用
- 规则2：引用必须是有效的

Rust 2018 引入了NLL(Non-Lexical Lifetime) 优化，编译器会分析“借用的实际使用范围”，而不仅仅是看花括号。基于此，我们可以将借用规则拓展如下：
- 规则1：一个变量声明共享引用时，它之前的所有可变引用失效
- 规则2：一个变量被读取时，它之前的所有可变引用失效
- 规则3：一个变量声明可变引用时，它之前的所有引用失效
- 规则4：一个变量被修改时（不是通过可变借用修改），它之前的所有引用失效
- 规则5：一个变量被移动或释放时，它的所有引用失效
### 3.1 共享借用将导致可变引用失效
- 变量被共享借用时，它之前的所有可变引用失效
```rust
    let mut v = 42;
    let r1: &mut i32 = &mut v;
    *r1 += 1;
    let r2: &i32 = &v;  //  共享借用，可变引用r1失效
    // *r1 += 1;        //  r1 不可再使用
```
### 3.2 值读取将导致可变引用失效
- 变量的值被读取后，它之前的所有可变引用失效
```rust
    let mut v = 42;
    let r = &mut v;
    *r += 1;
    let a = v;  //  v的值被读取用于按位拷贝生成a, 它的可变引用r失效
    // println!("{r}");    //  r 不可再使用
```
- 变量的值被读取后，原有的共享引用不受影响
```rust
    let mut v = 42;
    let r = &v;
    println!("{r}");
    let a = v;
    println!("{r}");    //  值被读取后原有的共享引用不受影响
```
&nbsp;
### 3.3 可变借用将导致所有引用失效
- 变量被可变借用时，它之前的所有共享引用或可变引用失效
```rust
    let mut v: i32 = 42;
    let r1: &i32 = &v;
    let r2: &i32 = &v;
    let r3: &i32 = &v;
    println!("{r1} {r2} {r3}");
    let r4: &mut i32 = &mut v;  // 可变借用，所有共享引用失效
    // println!("{r1} {r2} {r3}");  //  r1 r2 r3 不可再使用
    *r4 += 1;
    let r5: &mut i32 = &mut v;  // 可变借用，之前的可变引用r4失效
    // *r4 += 1;    //  r4 不可再使用
```
&nbsp;
### 3.4 值修改将导致所有引用失效
- 变量的值被修改时，它之前的所有共享引用或可变引用失效
```rust
    let mut v: i32 = 42;
    let r1: &i32 = &v;
    let r2: &i32 = &v;
    let r3: &i32 = &v;
    println!("{r1} {r2} {r3}");
    v += 1;  // 值修改，所有共享引用失效
    // println!("{r1} {r2} {r3}");  //  r1 r2 r3 不可再使用
    let r4: &mut i32 = &mut v;
    println!("{r4}");
    v += 1;  // 值修改，可变引用失效
    // println!("{r4}");  //  r4 不可再使用
```
&nbsp;
### 3.5 移动或释放将导致所有引用失效
- 变量被标记为moved或离开作用域时，它之前的所有共享引用或可变引用失效
```rust
    let mut s1 = String::from("hello");
    let r1: &String = &s1;
    println!("{r1}");
    let r2: &String;
    {
        let mut s2 = s1;
        // println!("{r1}");   //  s1被移动，r1不可再使用
        r2 = &s2;
        println!("{r2}");
    }
    // println!("{r2}");   //  s2作用域结束，r2不可再使用
```