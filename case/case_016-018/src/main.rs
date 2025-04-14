/*
 * 声明了可变引用后, 值的其他所有引用将失效
 * 值发生改变后，它的所有引用将失效
 * vec的元素发生任何的更改都会使得它的所有元素的引用失效
 * 推论: 1、值的可变引用至多存在一个    2、值的可变引用和不可变引用不可能同时存在  3、同一时刻只存在一种改变值的途径
 */ 
fn main() {
    /*
     * case 16
     * 声明了可变引用后, 值的其他所有引用将失效
     */
    println!("\ncase 16:");
    let mut num = 5;
    let ref1 = &num;
    let ref2 = &num;
    println!("{ref1} {ref2}");
    let ref3 = &mut num;    //  ref1 ref2 失效
    // println!("{ref1} {ref2}");
    println!("Compiling error!");
    println!("{ref3}");
    let ref4 = &mut num;  //   ref3 失效   
    // println!("{ref1} {ref2}");   //  使用了失效的引用
    // println!("{ref3}");          //  使用了失效的引用
    println!("Compiling error!");
    println!("{ref4}");

    /*
     * case 17 
     * 值发生改变后，它的所有引用将失效
     */
    println!("\ncase 17:");
    let mut num = 5;
    let ref1 = &num;
    let ref2 = &num;
    println!("{ref1} {ref2}");
    let ref3 = &mut num;    //  ref1 ref2 失效
    // println!("{ref1} {ref2}");
    println!("{ref3}");
    num += 1;                       //  ref3 失效
    // println!("{ref1} {ref2}");   //  使用了失效的引用
    // println!("{ref3}");          //  使用了失效的引用
    println!("Compiling error!");


    /*
     * case 18
     * vec的元素发生任何的更改都会使得它的所有元素的引用失效
     */
    println!("case 10:");
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    println!("Third element is {}", *num);
    v[0] = 0;        //  更改值
    v.push(4);       //  通过可变引用更改值
    println!("{v:?}");  
    // println!("Third element is {}", *num);   // 使用失效的可变引用
    println!("Compiling error!");
}
