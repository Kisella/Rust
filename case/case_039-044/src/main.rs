/*
 * case 44
 * 使用trait修复case 27
 */
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("case 44:");
    let num_list = [34, 50, 25, 100, 65];
    println!("{}", largest(&num_list));

    let char_list = ['y', 'm', 'a', 'q'];
    println!("{}", largest(&char_list));

    let str_list = vec![String::from("hello"), String::from("world")];
    println!("{}", largest(&str_list));

}
