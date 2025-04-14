#[derive(Debug, Clone, Copy)]
enum PathLCS {
    U,
    L,
    LU,
}

use PathLCS::*;

/// 计算两个序列的最长公共子序列 (LCS)
/// # 参数
/// - `x`: 第一个序列
/// - `y`: 第二个序列
/// # 返回值
/// - `(usize, Vec<T>)`: 包含长度和LCS本体的结果
pub fn longest_common_subsequence<T: PartialEq + Clone>(x: &[T], y: &[T]) -> Result<Vec<T>, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input sequences must be non-empty");
    }

    // 声明并初始化动态规划数组和路径数组
    let mut dp = vec![vec![0; y.len() + 1]; x.len() + 1];
    let mut path = vec![vec![U; y.len() + 1]; x.len() + 1];

    // 动态规划
    // dp[i][j] = x[i] == y[j] ? dp[i - 1][j - 1] + 1 : max(dp[i - 1][j], dp[i][j - 1])
    // path[i][j] = x[i] == y[j] ? LU : (dp[i - 1][j] >= dp[i][j - 1] ? U : L)
    for i in 1..=x.len() {
        for j in 1..=y.len() {
            if x[i - 1] == y[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                path[i][j] = LU;
            } else if dp[i - 1][j] >= dp[i][j - 1] {
                dp[i][j] = dp[i - 1][j];
                path[i][j] = U;
            } else {
                dp[i][j] = dp[i][j - 1];
                path[i][j] = L;
            }
        }
    }

    // 依靠追踪数据找出最长公共子序列
    let (mut i, mut j) = (x.len(), y.len());
    let mut indices = Vec::with_capacity(dp[i][j]); //  为索引数组分配内存
    while i != 0 && j != 0 {
        match path[i][j] {
            U => i -= 1,
            L => j -= 1,
            LU => {
                indices.push(i - 1);
                i -= 1;
                j -= 1;
            }
        }
    }
    let common_sub = indices.into_iter().rev().map(|i| x[i].clone()).collect();

    // 返回最长公共子序列的长度和本体
    Ok(common_sub)
}
