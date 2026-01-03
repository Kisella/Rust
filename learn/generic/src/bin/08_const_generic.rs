fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    let mut sum = 0;
    for i in 0..N {
        sum += arr[i];
    }
    sum
}

// 
fn sum_vector(arr: &[i32]) -> i32 {
    let mut sum = 0;
    let n = arr.len();
    for i in 0..n {
        sum += arr[i];
    }
    sum
}


fn main() {
    let arr10 = [1;10];
    let arr20 = [1;20];

    println!("sum10 = {}", sum_array::<10>(arr10));
    println!("sum20 = {}", sum_array::<20>(arr20));
}


/*
 * 编译器确定大小
 * 单态化为不同函数
 * 零运行时开销
 */
fn demo<const N: usize>() {
    let arr: [u8; N] = [0; N];
}

/*
 * 运行时确定大小
 * 堆区动态分配内存
 * 有一定性能开销
 */
fn demo_runtime(n: usize) {
    let arr= vec![0; n];
}