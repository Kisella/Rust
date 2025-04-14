/*
 * case 72
 * 要慎用move关键字，只有在必须取得其所有权时才使用，否则会得到不预期的结果
 * 没必要取得其所有权时，不使用move
 * 没有所有权可以取得时，不使用move
 */ 

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list1 = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut list2 = list1.clone();  //  复制一份list1以用于测试
    let mut num_sort_operations1 = 0;
    list1.sort_by_key(|r| {
        num_sort_operations1 += 1; // 在闭包里修改外部可变的变量，默认捕获的是被修改变量的可变引用，符合FnMut特性约束
        r.width
    });
    println!("{list1:#?}");
    println!("{num_sort_operations1}");


    let mut num_sort_operations2 = 0;
    list2.sort_by_key(move |r| {    //  强制闭包获得捕获变量的所有权
        num_sort_operations2 += 1; // 为了满足FnMut的特性约束，内部num_sort_operations2获得的是外部num_sort_operations2的副本(copy), 故对内部num_sort_operations2值的修改不会改变外部的值
        r.width
    });
    println!("{list2:#?}");
    println!("{num_sort_operations2}");     //  打印的是外部的num_sort_operations2，其结果为0不变

}