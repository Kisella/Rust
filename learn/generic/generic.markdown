# 泛型
---
泛型是一种抽象的类型参数，支持在编写函数、结构体、枚举时不必固定成某一种具体的类型
- 无泛型：重复劳动
```rust
fn swap_i32(a: i32, b:i32) -> (i32, i32) {
    (b, a)
}

fn swap_f64(a: f64, b:f64) -> (f64, f64) {
    (b, a)
}

fn swap_string(a: String, b:String) -> (String, String) {
    (b, a)
}
// 需要为每种类型重复编写几乎相同的代码
```
- 泛型: 一次编写
```rust
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}
// T 为类型参数，调用时可以是任何类型
```

## 1 泛型参数声明
- 函数名后在`<>`中声明泛型参数
- 参数列表、返回值、函数体内都可使用已声明的泛型参数
```rust
fn func<T, U, ...>(arg_1: T, arg_2: U ...) -> T {
    let var: T = ...;
    ...
}
// <T, U ...> 称为泛型参数声明，表示后续代码块{}中可以使用这些泛型参数。
```
```rust
fn swap_tuple<T, U>(tp: (T, U)) -> (U, T) {
    let t: T = tp.0;
    let u: U = tp.1;
    (u, t)
}

fn main() {
    let tp_1: (i32, String) = (15, String::from("hello"));
    let tp_2: (String, i32) = swap_tuple(tp_1);
}
```

## 2 单态化与自动推导
- 从泛型变为具体类型的过程称为**实例化**
- 编译器可以根据参数和返回值自动推导泛型实例化的具体类型
- 当编译器无法从上下文中推导出泛型实例化的类型时会报错
```rust
    fn echo<T>(x:T) -> T {
        x
    }
    //  参数类型确定，编译器自动推断T
    let a = echo(42); // T: i32
    let b = echo("hello"); // T: &str
```
```rust
    fn make_number<T>() -> T {
        panic!("something happened!")
    }
    // 返回值类型确定，编译器自动推断T
    let a: i32 = make_number();   //    T: i32
    let b: u8 = make_number();    //    T: u8
```

## 3 turbofish 语法
- 泛型只在函数体内部使用时，调用时必须使用`::<>`显式指定类型
```rust
    fn dosomething<T, U>() {
        println!("sizes: T = {}, U = {}",
            std::mem::size_of::<T>(),
            std::mem::size_of::<U>());
    }

    dosomething::<i32, u8>();
```

## 4 复合类型泛型
- 结构体和枚举支持使用泛型，让数据结构更具通用性
- 在复合类型名称后使用`<>`声明复合类型泛型
- 复合类型泛型不是一个具体的类型，而是一个类型族
```rust
    struct Name<T, U, ...> {
        item_1: T,
        item_2: U,
        ...
    }
```

```rust
    // 通过泛型定义一个复合类型泛型
    struct Point<T> {
        x: T,
        y: T,
    }

    // 自动推断为 Point<i32> 类型
    let p = Point {
        x: 0,
        y: 0,
    };

    // 显式指定为 Point<i32> 类型
    let p: Point<i32> = Point {
        x: 0,
        y: 0,
    };

    //  turbofish 指定为 Point<f64> 类型
    let f = Point::<f64> {
        x: 3.14,
        y: 3.14,
    };

    // Point<i32> 和 Point<f64> 是两个不同的类型
```
&nbsp;
- 支持使用`type`关键字为类型名定义别名
```rust
    type Pi32 = Point<i32>;
    let p: Pi32 = Point { x: 3, y: 4 };
    type ResultI32<E> = Result<i32, E>;
```
&nbsp;
## 5 泛型方法
- 为带泛型的类型实现方法时，需要在`impl`后声明泛型
```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    // 为Poin<T>类型族实现方法
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // 为Point<i32>类型实现方法
    impl Point<i32> {
        fn y(&self) -> &i32 {
            &self.y
        }
    }
```
&nbsp;
- 方法支持在函数名后声明额外的泛型参数
```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // 泛型参数<V, W>只在方法mixup内使用
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
```

可以把泛型参数实例化为具体类型，为特定类型实现专有方法
- 完全实例化
```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<i32> {
        fn print(&self) {
            println!("x: {}, y: {}", self.x, self.y);
        }
    }
```
- 部分实例化
- 确定的类型直接代替泛型，未确定的泛型要在`impl<>`中声明
```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T> Point<T, i32> {
        fn describe_y(&self) {
            println!("y: {}", self.y);
        }
    }

    impl<U> Point<i32, U> {
        fn describe_x(&self) {
            println!("x: {}", self.x);
        }
    }
```
&nbsp;
## 6 单态化与零成本抽象
Rust的泛型是一种零成本抽象，使用泛型不会带来任何性能的损失。在允许时Rust不会因为需要判断泛型参数是哪一种类型而导致效率变低，因为判断泛型在调用时属于哪一种类型在编译时就已经完成了，这一过程称为**单态化**。对于一个函数来说，他最终变为可执行程序时，每个参数和变量的类型都必须是确定的。而泛型是一个不确定的类型，当泛型出现时，一个泛型函数会在编译期通过单态化机制变为多个函数，泛型参数则会通过单态化变为多个类型，单态化结束后，每个函数和类型都是完全确定的。
```rust
fn swap<T, U>(a: T, b: U) -> (U, T) {
    (b, a)
}

// 调用
swap::<i32, i32>(1, 2);
swap::<u8, i32>(1 as u8, 2);
swap::<f64, f64>(3.14, 3.14);
```
编译器会生成三个不同版本的函数:
```rust
fn swap_i32_i32(a: i32, b: i32) -> (i32, i32) { (b, a) }
fn swap_u8_i32(a: u8, b: i32) -> (i32, u8) { (b, a) }
fn swap_f64_f64(a: f64, b: f64) -> (f64, f64) { (b, a) }
```
单态化的过程是按需进行的。泛型在调用时被实例化成了多少种类型，单态化机制就会按需产生对应数量的版本
```rust
    fn swap<T, U>(a: T, b: U) -> (U, T) {
        (b, a)
    }

    println!("{:p}", swap::<i32, i32> as fn (i32, i32) -> (i32, i32));
    println!("{:p}", swap::<u8, i32> as fn (u8, i32) -> (i32, u8));
    println!("{:p}", swap::<f64, f64> as fn (f64, f64) -> (f64, f64));
    // 三个函数指针的地址必定不同
```

## 7 const 泛型
- 在`<>`中使用`const`关键字定义const 泛型，支持在泛型中传入编译期的常量值
- 调用时通过`::<>`指定具体的const 泛型参数
- 函数和结构体的定义不仅可以依赖与类型，还可以依赖于赋值参数
```rust
fn func<const N: Type>() {
    // 函数体
}

struct Name<const N: Type> {
    // 字段
}
```
```rust
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += arr[i];
    }
    sum
}

fn sum_vector(arr: [i32; N]) -> i32 {
    let mut sum = 0;
    for i in 0..N {
        sum += arr[i];
    }
    sum
}

fn main() {
    let arr10 = [1;10];
    let arr20 = [1;20];

    println!("sum10 = {}", sum_array::<10>(arr10));
    println!("sum20 = {}", sum_array::<20>(arr20));
}
```
在编译器，函数的栈帧大小必须完全确定，这样程序运行这个函数时才能立刻为它分配合适的栈区空间。
```rust
fn sum_vector(arr: &[i32]) -> i32 {
    let mut sum = 0;
    let n = arr.len();
    for i in 0..n {
        sum += arr[i];
    }
    sum
}
// 如果不使用const泛型，则不能在编译器确定数组的大小，这种情况只能将arr替换为vector类型，将数据放在堆区, 产生运行时开销。
```
- const 泛型 VS 运行时参数
```rust
/*
 * 编译器确定大小
 * 单态化为不同函数
 * 零运行时开销
 */
fn demo<const N: usize>() {
    let arr: [u8; N] = [0; N];
}
```
```rust

/*
 * 运行时确定大小
 * 堆区动态分配内存
 * 有一定性能开销
 */
fn demo_runtime(n: usize) {
    let arr= vec![0; n];
}
```
&nbsp;
- 使用const泛型在编译期进行代码裁剪
```rust
    let x = 10;
    if x == 0 {
        println!("x is zero"); // 编译器识别到x不可能为0，不可能进入该分支，在生成汇编代码时直接将该分支裁剪掉。  
    } else {
        println!("x is not zero");
    }

    /* 裁剪后代码：
    let x = 10;
    println!("x is not zero");
     */
```
```rust
fn side_effect_true(buf: &mut [u8]) {
    buf[0] = 1;
}

fn side_effect_false(buf: &mut [u8]) {
    buf[0] = 2;
}

// 因为FLAG在编译器就能确定，编译器会裁剪掉不会执行的分支
fn test<const FLAG: bool>(buf: &mut [u8]) {
    if FLAG {
        side_effect_true(buf);
    } else {
        side_effect_false(buf);
    }
}

// flag在运行时才能确定，编译器未能进行裁剪
fn test_runtime(flag: bool, buf: &mut [u8]) {
    if flag {
        side_effect_true(buf);
    } else {
        side_effect_false(buf);
    }
}
```


const泛型是把“数值”提升到了和“类型”一样的维度，纳入泛型系统，这样Rust就能针对**类型**和**数值**两种参数来进行泛型化逻辑。
- 编译器为不同数值生成不同的函数版本
- 编译器配置内存布局，保证运行时不需要额外开销
- 实现零成本抽象时保证内存安全
- 可配合控制流进行编译器代码裁剪
&nbsp;
## 8 单态化的深层目的
单态化的目的，就是把未知信息在编译器变为已知
- 类型信息：某个类型能不能调用某个函数？
- 内存布局：类型和函数栈帧需要多少空间？
- 控制流：函数中的if分支能不能被裁剪掉？
- 数组长度：不同长度的静态数组需要不同的函数版本？
没有泛型和单态化，这些都需要程序员手动编写多个版本！
&nbsp;
## 9 小结
- 泛型函数使用`<T, U ...>`声明泛型参数
- 编译器可以自动推导或使用turbofish语法显式指定要实例化的类型
- 结构体、枚举支持使用泛型，泛型的不同实例化是不同的类型
- 为泛型类型实现方法需要在`impl`后声明泛型参数
- 可以通过完全或部分实例化为特定类型实现专有方法
- 单态化是零成本抽象的关键，按需生成具体版本的函数
- const泛型允许传入编译器常量，配合栈帧模型实现高效内存管理