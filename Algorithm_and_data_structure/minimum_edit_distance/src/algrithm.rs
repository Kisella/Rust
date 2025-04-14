#[derive(Debug, Clone, Copy)]
enum PathMED {
    LU, //  用t[j-1]替换s[i-1], 或无操作
    U,  //  删除s[i-1]
    L,  //  添加t[j-1]
}
use PathMED::*;

/// 最小编辑距离 (MED)
/// # 参数
/// - `s`: 原始串
/// - `t`: 目标串
/// # 返回值
/// `u32`: 最小操作次数
/// `Vec<String>`: 操作步骤
/// # 备注
/// 操作优先级 替换 > 删除 > 添加
pub fn minmum_edit_distance(s: &str, t: &str) -> (usize, Vec<String>) {
    if s.is_empty() && t.is_empty() {
        return (0, vec![]);
    }

    // 转化串为字符数组
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    // 声明和初始化动态规划数组与路径数组
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut path = vec![vec![LU; t.len() + 1]; s.len() + 1];
    for i in 0..=s.len() {
        dp[i][0] = i; //  把长度为i的串变为空串需要i次"删除"操作
        path[i][0] = U;
    }
    for j in 0..=t.len() {
        dp[0][j] = j; //  把空串变为长度为j的串需要j次"添加"操作
        path[0][j] = L;
    }

    // 动态规划
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if dp[i][j - 1] + 1 < dp[i - 1][j] + 1
                && dp[i][j - 1] + 1 < dp[i - 1][j - 1] + (s[i - 1] != t[j - 1]) as usize
            {
                // 添加操作
                dp[i][j] = dp[i][j - 1] + 1;
                path[i][j] = L;
            } else if dp[i - 1][j] + 1 < dp[i - 1][j - 1] + (s[i - 1] != t[j - 1]) as usize {
                // 删除操作
                dp[i][j] = dp[i - 1][j] + 1;
                path[i][j] = U;
            } else {
                // 替换或无操作
                dp[i][j] = dp[i - 1][j - 1] + (s[i - 1] != t[j - 1]) as usize;
                path[i][j] = LU;
            }
        }
    }

    // 回溯
    let (mut i, mut j) = (s.len(), t.len());
    let mut opera: Vec<String> = Vec::with_capacity(dp[i][j]);
    while i > 0 || j > 0 {
        match path[i][j] {
            LU => {
                i -= 1;
                j -= 1;
                if s[i] == t[j] {
                    continue;
                }
                opera.push(format!("第{}位的{}替换成{}", i, s[i], t[j]));
            }
            U => {
                i -= 1;
                opera.push(format!("删除第{}位的{}", i, s[i]));
            }
            L => {
                j -= 1;
                opera.push(format!("添加{}", t[j]));
            }
        }
    }
    opera = opera.into_iter().rev().collect();
    (dp[s.len()][t.len()], opera)
}

// leetcode 72. 编辑距离
pub fn min_distance(word1: String, word2: String) -> i32 {
    let s;
    let t;
    if word1.len() > word2.len() {
        s = word1.chars().collect::<Vec<_>>();
        t = word2.chars().collect::<Vec<_>>();
    } else {
        s = word2.chars().collect::<Vec<_>>();
        t = word1.chars().collect::<Vec<_>>();
    }
    let (m, n) = (s.len(), t.len());
    if m == 0 && n == 0 {
        return 0;
    }
    // 选择长度较小的串放入t，进一步减小内存
    let mut dp_prev = vec![0; n + 1];
    let mut dp_curr = vec![0; n + 1];

    for j in 0..=n {
        dp_prev[j] = j;
    }

    // 动态规划
    for i in 1..=s.len() {
        dp_curr[0] = i;
        for j in 1..=t.len() {
            dp_curr[j] = (dp_prev[j] + 1)
                .min(dp_curr[j - 1] + 1)
                .min(dp_prev[j - 1] + (s[i - 1] != t[j - 1]) as usize);
        }
        std::mem::swap(&mut dp_prev, &mut dp_curr);
    }
    dp_prev[n] as i32
}

// pub fn min_distance(word1: String, word2: String) -> i32 {
//     let s = word1.chars().collect::<Vec<_>>();
//     let t = word2.chars().collect::<Vec<_>>();
//     if s.is_empty() && t.is_empty() {
//         return 0;
//     }

//     let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
//     for i in 0..=s.len() {
//         dp[i][0] = i; //  把长度为i的串变为空串需要i次"删除"操作
//     }
//     for j in 0..=t.len() {
//         dp[0][j] = j; //  把空串变为长度为j的串需要j次"添加"操作
//     }

//     // 动态规划
//     for i in 1..=s.len() {
//         for j in 1..=t.len() {
//             dp[i][j] = (dp[i - 1][j] + 1)
//                 .min(dp[i][j - 1] + 1)
//                 .min(dp[i - 1][j - 1] + (s[i - 1] != t[j - 1]) as usize);
//         }
//     }
//     dp[s.len()][t.len()] as i32
// }
