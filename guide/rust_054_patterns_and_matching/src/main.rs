/*
 * @绑定
 * @符号可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存这个值
 */

enum Message {
    Hello { id: i32 },
}
enum Message1 {
    Hello(i32),
}
fn main() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    let msg = Message1::Hello(5);
    match msg {
        Message1::Hello(id_variable @ 3..7) => {
            println!("Found an id in range: {}", id_variable)
        }
        Message1::Hello(10..=12) => {
            println!("Found an id in another range")
        }
        Message1::Hello(id) => {
            println!("Found some other id: {}", id)
        }
    }
}
