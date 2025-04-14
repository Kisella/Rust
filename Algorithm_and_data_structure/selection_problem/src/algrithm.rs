use rand::{Rng, thread_rng};
use std::cmp::Ordering::*;

// 次序选择问题
pub fn select<T: PartialEq + PartialOrd + Clone>(
    array: &mut [T],
    order: usize,
) -> Result<&T, &'static str> {
    if order < 1 || order > array.len() {
        return Err("Order must be in 1..=array.len()");
    }

    let mut rng = thread_rng();
    let mut arr = array;
    let mut target = order - 1; // 转换为 0-based 索引

    loop {
        let arr_len = arr.len();
        if arr_len == 0 {
            break Err("Unexpected empty subarray");
        }

        // 随机选择主元并交换到首位
        let p_idx = rng.gen_range(0..arr_len);
        arr.swap(0, p_idx);
        let p = arr[0].clone();

        // Hoare 划分法
        let (mut i, mut j) = (1, arr_len - 1);
        while i <= j {
            while i <= j && arr[i] <= p {
                i += 1;
            }
            while i <= j && arr[j] > p {
                j -= 1;
            }
            if i <= j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        arr.swap(0, j); // 主元归位到 j

        match target.cmp(&j) {
            Less => arr = &mut arr[..=j - 1], //  左迭代
            Greater => {
                arr = &mut arr[j + 1..]; //  右迭代
                target -= j + 1;
            }
            Equal => break Ok(&arr[j]), //  命中
        }
    }
}

// fn QuickSort<T: PartialOrd + Clone>(array: &mut [T], left: usize, right: usize) {
//     if left >= right {
//         return;
//     }
//     let p_idx = thread_rng().gen_range(left..=right);
//     let p = array[p_idx].clone();
//     array[p_idx] = array[left].clone();
//     array[left] = p.clone();
//     let (mut i, mut j) = (left, right);
//     while i < j {
//         while i < j && array[j] >= p {
//             j -= 1;
//         }
//         if i < j {
//             //  遇到 array[j] < p, 需要左移
//             array[i] = array[j].clone();
//             i += 1;
//         }
//         while i < j && array[i] < p {
//             i += 1;
//         }
//         if i < j {
//             //  遇到 array[i] >= p, 需要右移
//             array[j] = array[i].clone();
//             j -= 1;
//         }
//     }
//     array[i] = p; // i == j
//     QuickSort(array, left, i - 1);
//     QuickSort(array, i + 1, right);
// }
