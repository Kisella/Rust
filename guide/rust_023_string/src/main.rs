fn main() {
    let s1 = String::from("hello");
    // let h = s1[0];      //  Compilying error
    //  String类型不支持通过索引访问元素

    //  len方法获取字符串所占的字节数
    let len = String::from("Hola").len();
    println!("{}", len);

    // bytes方法获取字符串的的所有字节
    let s1 = "ABCDE01234";
    for b in s1.bytes() {
        println!("{b}");
    }

    // chars方法获取字符串的所有字符(Unicode标量值)
    for b in s1.chars() {
        println!("{b} ");
    }

    // 可以使用中括号([])和一个范围来创建字符串的切片
    let s = "520 I love you!".to_string();
    let s1 = &s[4..14];
    println!("{s1}");
    // 注意：如果切割时跨越了字符边界，程序就会panic
}
