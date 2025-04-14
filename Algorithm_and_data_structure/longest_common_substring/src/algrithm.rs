// 最长公共字数组
// 滚动变量法改进，空间复杂度优化到O(n)水平
pub fn longest_common_substring<'a, T: PartialEq>(x: &'a [T], y: &[T]) -> &'a [T] {
    if x.is_empty() || y.is_empty() {
        return &x[0..0];
    }

    // 声明并初始化动态规划数组
    let mut prev_dp = vec![0; y.len() + 1];

    // 动态规划
    // curr_dp[j] = (x[i - 1] != y[j - 1] ? 0 : prev_dp[j - 1] + 1)
    let (mut end_pos, mut max_len) = (0, 0);
    for i in 1..=x.len() {
        let mut curr_dp = vec![0; y.len() + 1]; // 当前行
        for j in 1..=y.len() {
            if x[i - 1] != y[j - 1] {
                curr_dp[j] = 0;
                continue;
            }
            curr_dp[j] = prev_dp[j - 1] + 1;
            if curr_dp[j] > max_len {
                max_len = curr_dp[j];
                end_pos = i - 1;
            }
        }
        prev_dp = curr_dp;  // 滚动更新
    }

    //  返回最大切片
    if max_len == 0 {
        &x[0..0]
    } else {
        &x[end_pos - max_len + 1..=end_pos]
    }
}

// pub fn longest_commom_substring<'a, T: PartialEq>(x: &'a [T], y: &[T]) -> &'a [T] {
//     if x.is_empty() || y.is_empty() {
//         return &x[0..0];
//     }

//     // 声明并初始化动态规划数组
//     let mut dp = vec![vec![0; y.len() + 1]; x.len() + 1];

//     // 动态规划
//     // dp[i][j] = (x[i - 1] != y[j - 1] ? 0 : dp[i - 1][j - 1] + 1)
//     let (mut end_pos, mut max_len) = (0, 0);
//     for i in 1..=x.len() {
//         for j in 1..=y.len() {
//             if x[i - 1] != y[j - 1] {
//                 dp[i][j] = 0;
//                 continue;
//             }
//             dp[i][j] = dp[i - 1][j - 1] + 1;
//             if dp[i][j] > max_len {
//                 max_len = dp[i][j];
//                 end_pos = i - 1;
//             }
//         }
//     }

//     //  返回最大切片
//     if max_len == 0 {
//         &x[0..0]
//     } else {
//         &x[end_pos - max_len + 1..=end_pos]
//     }
// }