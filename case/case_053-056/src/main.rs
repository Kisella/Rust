/*
 * case 53
 *  结构体中的字段中存在引用时，需要为其标识生命周期参数
 */
#[derive(Debug)]
struct ImportantExcerpt<'a, 'b> {
    // 这里的结构体ImportantExcerpt声明了两个生命周期参数，而其实例的生命周期就是'a 和 'b 之间的较小者
    part1: &'a str,
    part2: &'b str,
}

/*
 * case 54
 * 为含有生命周期参数的结构体定义方法
 * impl关键字后需要带有生命周期参数的声明
 */
impl<'a, 'b> ImportantExcerpt<'a, 'b> {
    /*
     * case 55
     * 当方法都没有必要标注生命周期参数时，返回值的引用默认来源于self
     */
    fn annouce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part1
    }

    /*
     * case 56
     * 当方法的返回值引用不来源于self而来源于其他入参时，就需要为方法声明生命周期参数
     * 注意：声明的生命周期参数不能是impl关键字后声明过的，否则将造成错误的shadow
     */
    fn annouce_and_return_annouce<'c>(&self, announcement: &'c str) -> &'c str {
        println!("Attention please: {}", self.part1);
        announcement
    }
}

fn main() {
    println!("case 53");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let hello = String::from("Hello world");
    let i = ImportantExcerpt {
        part1: &novel,
        part2: &hello,
    };
    println!("{i:#?}");
    drop(novel);
    println!("{}", &hello);
    // println!("{}", i.part2);    //  Compiling error, 使用了失效后的实例。结构体的字段part1失效，那整个结构体的实例也就失效了，即使它的part2字段依然有效

    println!("\ncase 54 & 55 & 56");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let hello = String::from("Hello world");
    let i = ImportantExcerpt {
        part1: &novel,
        part2: &hello,
    };
    i.annouce_and_return_part("mother fuck");
    i.annouce_and_return_annouce("mother fuck");
}
