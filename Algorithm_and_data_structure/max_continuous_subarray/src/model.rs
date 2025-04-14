// 最大子数组问题：
//     给定一个数组X，它的子数组为数组X中连续的一段序列。求数组X的最大非空子数组。
// pub fn max_continuous_subarray(array: &[i32]) -> (i32, (usize, usize)) {
//     if array.is_empty() {
//         panic!("Input array cannot be empty");
//     }

//     // 声明动态规划数组，长度匹配目标数组，统一初始化为目标数组的最后一个元素
//     let mut dp = vec![array[array.len() - 1]; array.len()];

//     // 声明路径数组，长度匹配目标数组，统一初始化为目标数组长度减1(最后一个元素的索引)
//     let mut path = vec![array.len() - 1; array.len()];

//     // 动态规划，从array倒数第二个元素开始遍历到开头
//     // dp[i] = (dp[i+1] > 0) ? array[i] + dp[i+1] : array[i]
//     // path[i] = (dp[i+1] > 0) ? path[i] : i
//     let (mut start_index, mut max_sum) = (0, array[0]);
//     for i in (0..array.len() - 1).rev() {
//         if dp[i + 1] > 0 {
//             dp[i] = array[i] + dp[i + 1];
//             path[i] = path[i + 1];
//         } else {
//             dp[i] = array[i];
//             path[i] = i;
//         }
//         if dp[i] > max_sum {
//             (start_index, max_sum) = (i, dp[i]);
//         }
//     }
//     // println!("{dp:?} \n {path:?}");
//     (max_sum, (start_index, path[start_index]))
// }

// pub fn max_continuous_subarray(array: &[i32]) -> (i32, (usize, usize)) {
//     if array.is_empty() {
//         panic!("Input array cannot be empty");
//     }

//     //  声明并初始化动态规划滚动变量
//     // 使用滚动变量法进行改进，空间复杂度优化到常数级
//     // dp[i] => current_sum
//     // path[i] => end, temp_end
//     let last_index = array.len() - 1;
//     let (mut max_sum, mut current_sum) = (array[last_index], array[last_index]);
//     let (mut start, mut end, mut temp_end) = (last_index, last_index, last_index);

//     // 动态规划，从array倒数第二个元素开始遍历到开头
//     // dp[i] = array[i] + dp[i + 1]  ==>    current_sum += num;
//     //        path[i] = path[i + 1]  ==>    直接消去
//     //             dp[i] = array[i]  ==>    current_sum = num
//     //                 path[i] = i   ==>    temp_end = i;
//     for (i, &num) in array.iter().enumerate().rev().skip(1) {
//         if current_sum > 0 {
//             current_sum += num;
//         } else {
//             current_sum = num;
//             temp_end = i;
//         }
//         if current_sum > max_sum {  //  此处可添加边界以获得最长或者最短的字串(多解情况)
//             max_sum = current_sum;
//             start = i;
//             end = temp_end;
//         }
//     };
//     (max_sum, (start, end))
// }

// 最大子数组问题
pub fn max_continuous_subarray(array: &[i32]) -> (i32, (usize, usize)) {
    if array.is_empty() {
        panic!("Input array cannot be empty");
    }

    //  声明并初始化动态规划滚动变量
    // 使用滚动变量法进行改进，空间复杂度优化到常数级
    let (mut max_sum, mut cur_sum) = (array[0], array[0]);
    let (mut start, mut end, mut temp_start) = (0, 0, 0);

    for (i, &num) in array.iter().enumerate().skip(1) {
        if cur_sum > 0 {
            cur_sum += num;
        } else {
            cur_sum = num;
            temp_start = i;     //  探索者
        }
        if cur_sum > max_sum {  //  此处可添加边界以获得最长或者最短的字串(多解情况)
            max_sum = cur_sum;
            end = i;
            start = temp_start;
        }
    };
    (max_sum, (start, end))
}
