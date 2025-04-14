use std::backtrace;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        //      使用match匹配枚举
        Message::Quit => {
            println!("The Quit variant has no data to destrucuture.")
        }
        Message::Move { x, y } => {
            //  匹配变体Move并获得其字段值
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message:{}", text), //  匹配变体Write, 并获得其元组值
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 结构嵌套的枚举
    let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        // 方式一: 
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message1::ChangeColor(Color::Hsv(h,s ,v )) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => (),

        // 方式二: (可读性差，嵌套深，不推荐)
        // Message1::ChangeColor(color) => {
        //     match color {
        //         Color::Rgb(r,g ,b ) => { 
        //             println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        //         }
        //         Color::Hsv(h,s ,v ) => {
        //             println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        //         }
                
        //     }
        // }
        // _ => (),
    }
}
