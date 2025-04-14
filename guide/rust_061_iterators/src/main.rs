/*
 * 迭代器允许你依次对一系列项目(item)执行某些操作
 * rust的迭代器是惰性的，除非调用某些方法来消耗这些迭代器，否则迭代器是不会立即生效的
 */ 
fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    // 将迭代器用于for循环时，迭代器中的每个元素在循环的每一次，都会被使用
    for val in v1_iter {    //  for循环会获取迭代器v1_iter的所有权，并在内部自动设为可变(mut)的
        println!("Got: {val}");
    }
}
