# Rust代码组织架构

## 1 package
包含cargo.toml文件和crate的目录称为Rust的一个package, 它是Rust组织和共享的重要单元，通常包含一个或多个crate。
- 至少有一个crate
- 可以有多个binary crate
- 最多只能有一个libratry crate

使用`cargo new $packagename`可以创建一个package
```sh
cargo new $packagename  # 生成一个package, 包含一个binary crate
cargo new $packagename --lib # 生成一个package, 包含一个library crate
```


## 2 crate
Crate 是 Rust 中编译的基本单元和代码分发的基本单位。在 Rust 的世界里，一个 crate 可以看作是一个独立的项目或库。
### 2.1 分类
- binaty crate 有main函数的, 可执行的
- library crate 没有main函数，无法执行。定义一些功能，可共享使用
### 2.2 crate root
crate root 是编译crate的入口点
- binary crate: src/main.rs, src/bin/*.rs
- library crate: src/lib.rs

### 2.3 bin
在package的src目录下创建一个bin目录，然后在bin目录中为每个binary crate创建一个.rs文件。每个.rs文件都会被编译成一个独立的可执行文件, 即/src/bin/目录下的每个.rs文件都是一个binary crate。


- 目录结构
```txt
my_project
├── Cargo.toml
└── src
    ├── main.rs    # 默认的binary crate
    ├── lib.rs     # library crate（如果有的话）
    └── bin
        ├── binary1.rs
        ├── binary2.rs
        └── ...
```

- 构建与运行
```sh
# 运行默认的 main.rs
cargo run

# 运行特定的 binary crate
cargo run --bin tool1
cargo run --bin tool2 -- Alice
cargo run --bin helper

# 构建所有 binary crates
cargo build
cargo build --bin tool1
cargo build --bin tool2

# 安装到 cargo bin 目录
cargo install --path . --bin tool1
```
&nbsp;
如果有需要，可在在`Cargo.toml`中配置多个binary target
```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

# 默认的binary crate是src/main.rs，所以以下配置是可选的，但为了清晰可以写明
[[bin]]
name = "main-app"
path = "src/main.rs"

[[bin]]
name = "custom-tool"
path = "tools/custom_main.rs"

[[bin]]
name = "cli-app"
path = "src/bin/cli.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
```
对应目录结构为：
```txt
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs          # 作为 main-app
│   └── bin/
│       └── cli.rs       # 作为 cli-app
├── tools/
│   └── custom_main.rs   # 作为 custom-tool
```

## 3 Module
Rust 中的模块 (Module) 是代码组织的基本单位，用于控制代码的可见性（公有和私有）和命名空间。模块允许你将相关的函数、结构体、枚举、常量等组织在一起，并可以嵌套形成层次结构。

### 3.1 内联模块 (Inline Module)
在文件中使用`mod`关键字定义内联模块
```rust
mod $module_name {代码块}   # 定义内联模块
```
示例:
```rust
mod math {
    pub mod basic {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}
fn main() {
    println!("2 + 3 = {}", math::basic::add(2, 3));
}
```

### 3.2 文件模块
假设有一个模块 my_module，那么可以创建一个文件 my_module.rs，并在其中定义模块内容。

```rust
// my_module.rs
pub fn hello() {
    println!("Hello from my_module!");
}
```
然后在主文件（如 main.rs）中声明并导入这个模块：
```rust
// main.rs
mod my_module;
fn main() {
    my_module::hello();
}
```

注意，使用这种方法定义的模块，定义文件和引用的文件需在同一级目录
```txt
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs          # 引用my_module
│   └── my_module.rs     # 定义my_module

```

### 3.3 目录模块
如果模块内容较多，可以将其放在一个目录中，并在该目录的同级创建与目录同名的.rs文件。例如创建`my_models.rs`文件和目录`my_models/`, `my_models`是一个目录模块，`my_models/`目录下可以定义`my_models`的子模块。并将子模块的声明写在`my_models.rs`中。
&nbsp;
示例：

```rust
// /my_project/src/bin/my_models.rs
pub mod enums;    // 创建模块文件my_models.rs，并声明子模块enums
```
```rust
// /my_project/src/bin/my_models/enums.rs
// 创建目录my_models，再在my_models目录下创建子模块enums
#[derive(Debug)]
pub enum YesNo {
    YES,
    No
}
```
```rust
// /my_project/src/bin/main.rs
// 在main.rs下声明my_models，并使用my_model的子模块enums
mod my_models;
fn main() {
    let y = my_models::enums::YesNo::YES;
    println!("{y:?}");
}
```
对应目录结构为：
```txt
my_project/
├── Cargo.toml
├── src/
|   ├──bin/
|   |   ├── my_module/
|   |   |  ├── sub_module1.rs
|   |   |  ├── sub_module2.rs
|   |   |  ...
|   |   |
|   |   ├── my_module.rs # 定义模块my_module中的子模块
|   |   └── main.rs   
```
&nbsp;
### 3.4 模块路径
在Rust中，模块路径是用于引用模块树中特定项的路径，是代码组织和访问的核心机制。它定义了如何在模块树中定位和引用代码项。模块路径可以分为两种：绝对路径和相对路径。
在 Rust 中，`crate::` 是一个特殊的关键字，用于表示当前 crate 的根模块。对于一个binary crate, 它的`main`函数就在crate的根模块中。可以以`crate::`开始逐级找到crate中的所有模块。从crate根开始的路径称为绝对路径。
&nbsp;
`self::`表示当前模块，`super::`表示父模块，从当前模块或父模块开始的路径称为相对路径。`self::`可以省略，直接以标识符开始调用当前作用域的项。
&nbsp;
可以`$package_name::`开始访问某个package的library crate中定义的模块。
&nbsp;
示例：
```rust
// main.rs
fn main() {
    crate::m1::sub_m1::method1();  //    绝对路径
    crate::m2::sub_m2::method2();
    crate_guide::m3::sub_m3::method3(); // 调用library crate中的模块
}

mod m1 {
    pub mod sub_m1 {
        pub fn method1() {
            println!("Method 1");
        }
    }
}

mod m2;    //  模块定义在m2.rs中

mod m1 {
    pub mod sub_m1 {
        pub fn method1() {
            println!("Method 1");
        }
    }
}

mod m2;    //  模块定义在m2.rs中
```
```rust
// m2.rs
pub mod sub_m2 {
    pub fn method2() {
        println!("method2 call method1");
        super::super::m1::sub_m1::method1();   // 相对路径
    }
}
```
```rust
// lib.rs
pub mod m3 {
    pub mod sub_m3 {
        pub fn method3() {
            println!("method3");
        }
    }
}
```

对应目录结构为：
```txt
my_project/
├── Cargo.toml
├── src/
|   ├──bin/
|   |   ├── my_crate/
|   |      ├── main.rs
|   |      └── m2.rs
|   └── lib.rs
my_module    
```
### 3.5 引用路径
模块中使用`use`关键字引入路径，可减少模块路径的代码
- 使用`{}`可引用模块中的多个项目
- 使用`*`可引用模块中的所有项目或enum中的所有变体
```rust
mod m1 {
    pub mod sub_m1 {
        use super::super::m2::SM;
        pub fn function(zero: SM, one :SM) {
            println!("Ask for help, please call {one:?}, {one:?}, {zero:?}");
        }
    }
}

mod m2 {
    #[derive(Debug)]
    pub enum SM {
        ZERO,
        ONE,
    }
}

use m1::sub_m1::*;
use m2::SM::*;

fn main() {
    let kiana = ZERO;   //  可以直接使用变体名
    let mei = ONE;
    function(kiana, mei); // 可以直接使用函数名调用
}
```

&nbsp;
- 当引用的多个项目中存在同名时，需要使用`as`关键字对项目起别名
```rust
use std::fmt::Result;
use std::io:: Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult {
    // -- snip--
}
```
&nbsp;
- 父模块为私有，子模块为公有。可使用`pub use`导出子模块
- 使用`pub use`可以使我们精细决定哪些项目和模块对外公开，同时保持避免一些不必要的父类公开。

### 3.6 可见性
- Rust中的项目（模块，函数，数据结构等）默认是private(私有)
- 使用`pub`关键字，可以让项变为public(公有)，使其他项目能够使用
- 子模块可以使用其祖先模块中的项
- 父模块中的项目不能使用子模块中的私有项
- 结构体本身的可见性和其字段的可见性是分开的。需为结构体本身和各字段单独设置`pub`
- 枚举本体和其变体的可见性是统一的。

```rust
pub struct Breakfast { // public
    pub toast: String,  //  public
    seasonal_fruit: String // private
}

pub enum Appetizer {  // public
    Soup,      // public
    Salad,     // public
}
```