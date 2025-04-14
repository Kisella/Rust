fn main() {
    /*
     * case 24 truth
     * 元组中的元素具有独立性
     */ 
    println!("case 25");
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = & name.0;   
    name.1.push_str("Esq.");    //  name.1 的可变引用不会影响到 name.0的引用。name.0的不可变引用依然有效
    println!("{first} {}", name.1);

    /*
     * case 24 faker 
     * 对元组整体的引用会打破元素引用之间的独立性
     */ 
    println!("\ncase 26");
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = get_first(&name);  // diff from case 25 
    // Rust目前只能做到关心函数的签名，通过get_first的签名rust可以推断出它是借用了name中某个元素。但借用的是元素0还是元素1，rust无法区分。这样一来rust也就无法推断出first是哪个元素的不可变引用了。
    name.1.push_str("Esq.");   //   name.1 的可变引用，使得first失效(虽然first事实是&name.0, 但是rust推断不出来，rust的保守策略迫使first失效)
    // println!("{first} {}", name.1);  //  使用了失效的引用
    println!("Compiling error!");

    /*
    * case 25 
    * 数组中的元素具有耦合性，对其中一个元素的引用会使得其他元素的引用失效
    */ 
    println!("case 27");
    let mut a = [0,1,2,3];
    let x = &mut a[1];
    *x += 1;
    let y = &a[2];      //  对a[2]的不可变引用使得a[1]的可变引用失效
    // *x += *y;      //   解引用了失效的引用x
    println!("Compiling error!");
    println!("{a:?}");
}

fn get_first(name: &(String,String)) -> &String {
    &name.0     //  实际返回的是name.0的引用，但目前rust还做不到关心函数的内部实现，只能利用函数签名进行有限推断
}
