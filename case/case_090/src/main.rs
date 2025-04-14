use std::{fs::File, io::ErrorKind};
fn main() {
    /*
     * case 90
     * 调试模式下命令行SET RUST_BACKTRACE=full查看恐慌时的完整调用信息
     */ 
    // let v = vec![1,2,3];
    // v[99];

    /*
     * case 91
     * 使用Result<T, E>处理可能的错误
     * 配合使用多层嵌套match以解构Result，进行错误处理
     */ 
    let greeting_file_result = File::open("hello.txt");
    // 成功: 返回Ok(T), T: std::fs::File  文件句柄
    // 失败: 返回Err(E), E: std::io::Error 
    let greet_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }
            other_error => panic!("Problem opening the file: {other_error}")
        },
    };
    println!("{greet_file:?}");

    /*
     * case 92
     * 使用unwrap()或expect()进行快速解构
     * unwrap()用于提取Option或Result类型内部的值，如果是None或Err则恐慌
     * expect()与unwrap()类似，并支持提供一个自定义的panic信息
     */ 
    let greeting_file_result = File::open("hello.txt").unwrap();
    let greeting_file_result = File::open("hello.txt").expect("File opened fail!");
    

}
