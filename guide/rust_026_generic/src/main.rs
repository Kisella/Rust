fn main() {
    let number_list = [34,50,25,100,65];
    let result = largest(&number_list);
    println!("{:?}",number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102,34,6000,89,54,2,43,8];
    let result = largest(&number_list);
    println!("{:?}",number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("{:?}",char_list);
    println!("The largest char is {}", result);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("{:?}",str_list);
    println!("The largest str is {}", result);

    println!("");

    let number_list = [34,50,25,100,65];
    let result = largest1(&number_list);
    println!("{:?}",number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102,34,6000,89,54,2,43,8];
    let result = largest1(&number_list);
    println!("{:?}",number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];
    let result = largest1(&char_list);
    println!("{:?}",char_list);
    println!("The largest char is {}", result);

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest1(&str_list);
    println!("{:?}",str_list);
    println!("The largest str is {}", result);
}

// fn largest<T>(list:&[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }    //    Compiling error

fn largest<T:PartialOrd + Clone>(list:&[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}
/* 
 * 1、只有T类型实现了PartialOrd这个Trait才能使用大括号进行比较 => 在参数T中添加PartialOrd这个Trait约束
 * 2、T类型没有实现Copy这个trait, 故无法从list[0]移出元素 => 参数T中添加Copy这个Trait约束，或者为进一步提高泛用性，选择添加Clone这个Trait约束, 然后list[0]调用Clone方法
 * 3、for &item in list.iter() 的写法需要所有权发生移动，需要实现了Copy这个trait => 去掉item前的'&'
 * 4、做完3后使得 item 和 largest 不匹配 => item解引用或者largest借用
 * 5、=> 在赋值给largest时item调用Clone方法
 */

// 或者可以直接返回一个T的引用，这样就不用克隆了
fn largest1<T:PartialOrd>(list:&[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}  
/* 
 * 1、在参数T中添加PartialOrd这个Trait约束
 * 2、返回值T加上引用
 * 3、list[0]无需克隆，直接引用即可
 */  