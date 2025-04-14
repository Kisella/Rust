#[derive(Debug)]
struct ImportantExcerpt<'a> {   //  在结构体名称后声明生命周期注解，这样即可在结构体定义里使用对应的注解
    part: &'a str,
}
// 这表明ImportantExcerpt类型的实例的生命周期不能超出其持有字段part的生命周期，若part字段所引用的字符串失效了，那么该ImportantExcerpt的实例也就失效了，否则将导致悬垂引用

fn main() {
    let novel = String::from("Call me Ishmal. Some years ago...");
    let mut index = novel.split('.');
    let first_sentence = index.next().unwrap();
    let second_sentence = index.next().unwrap();
    println!("{second_sentence}");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:#?}", i);
}
