fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 只要模式继续满足匹配的条件，那他允许while循环一直运行
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
