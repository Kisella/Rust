/*
 * 没有任何限制的泛型T很多操作无法进行
 */ 

/*
 * case 34
 * 没有任何限制的泛型参数T是无法打印的, 因为T没有实现Display特性
 */
fn print_slice<T>(v: &[T]) {
    for x in v {
        println!("{x}");
    }
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            //  没有任何限制的泛型参数T是无法比较大小的
            largest = item;
        }
    }
    largest
}

fn main() {
    print_slice(&[1, 2, 3]);
    println!("Compiling error!");
}
