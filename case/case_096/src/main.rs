/*
 * case 96
 * 通过实现From特性返回自定义的错误类型
 */  
use std::{
    fs::File,
    io::{Error, Read},
    num::ParseIntError,
};

// 声明一个自定义的错误类型
#[derive(Debug)]
pub enum MyError {
    Io(Error),
    ParseInt(ParseIntError),
    Other(String),
}

// 为MyError类型实现From<io::Error>特性，使得io类型的错误可以通过?转化为MyError
impl From<Error> for MyError {
    fn from(value: Error) -> Self {
        MyError::Io(value)
    }
}

// 为MyError类型实现From<num::ParseIntError>特性，使得ParseIntError类型的错误可以通过?转化为MyError
impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> Self {
        MyError::ParseInt(value)
    }
}

fn read_username_from_file() -> Result<String, MyError> {
    let mut name = String::new();
    let file = File::open("some.txt")?.read_to_string(&mut name)?;
    let num: i32 = "55".parse()?;
    Ok(name)
}

/*
 * case 97
 * main函数返回Result<T, E>
 */  
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

/*
 * case 98 faker
 * ?运算符类型不匹配
 */ 
// File::open 返回 Result<File, std::io::Error>
// read_to_string 返回 Result<usize, std::io::Error>
// 但函数返回 Option<String>，导致 Result 和 Option 类型不兼容
fn read_user_name_from_file() -> Option<String> {
    let mut username_file = File::open("Hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Some(username)
}
