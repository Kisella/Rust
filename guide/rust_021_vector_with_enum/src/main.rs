#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // vector可以配合enum使用来存储多个数据类型
    println!("{:?}", row);

    // 遍历vector
    for v in row.iter() {
        match v {
            SpreadsheetCell::Float(f) => println!("{}",f),
            SpreadsheetCell::Int(i) => println!("{}",i),
            SpreadsheetCell::Text(str) => println!("{}",str),
        }
    }
}
