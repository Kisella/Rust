use std::{fs::File, io::{self, Read}};
/*
 * case 093
 * Result<T>作为返回值
 * 将错误返回，由调用该函数的代码来决定如何处理错误
 */ 

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username:String = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/*
 * case 94
 * 问号(?)运算符
 * --如果操作成功，解包Ok并执行下一行代码
 * --操作失败，返回Err，并将错误传播给调用者
 * 使用?运算符可以减少大量的match和if let语句，使代码更简洁
 */ 
fn read_username_from_file_brief() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // 可简化为:
    let mut username_file = File::open("hello.txt")?;

    let mut username:String = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    // 可简化为:
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/*
 * case 95
 * 问号(?)运算符链式调用
 */ 

 fn read_username_from_file_final() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username:String = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)
    // 通过链式调用节省中间变量username_file，进一步简化
    let mut username:String = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    println!("Hello, world!");
}
