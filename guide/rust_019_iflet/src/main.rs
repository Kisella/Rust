fn main() {
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => {}
    }

    if let Some(3) = v {
        println!("three");
    }
    // if let 相当与只对一种匹配模式进行处理，不关心其他的情况
    // if let 只针对一种匹配模式进行处理，放弃了穷举的可能，换来了更少的代码，更少的缩进，更少的模板代码

    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
    // if let 可以搭配else、eles if 、eles if let使用


    let favorite_color:Option<&str> = None;
    let is_tuesday = false;
    let age:Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color {   //  模式匹配，可将Some变体中的值取出
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {       //  模式匹配，可将Ok变体中的age值取出, 赋值给新变量age
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

