use std::cmp::max;

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

// 0-1背包问题
#[derive(Debug, Clone, Copy)]
pub struct Commodity {
    pub name: &'static str,
    pub price: u32,
    pub volume: usize,
}
pub fn knapsack(commodity_list: &[Commodity], capacity: usize) -> (u32, Vec<usize>) {
    if capacity == 0 || commodity_list.is_empty() {
        return (0, vec![]);
    }

    // 声明并初始化动态规划数组
    let mut dp = vec![0; capacity + 1];

    // 根据递推公式，自底向上计算，得到最优解
    for item in commodity_list.iter() {
        for c in (item.volume..=capacity).rev() {
            dp[c] = max(dp[c - item.volume] + item.price, dp[c])    //  递推公式
        }
    }

    // 回溯法重建路径，得到选择结果
    let mut remaining = capacity;
    let mut selected = vec![];
    for (idx, item) in commodity_list.iter().enumerate() {
        if remaining >= item.volume && dp[remaining] == dp[remaining - item.volume] + item.price {
            selected.push(idx);
            remaining -= item.volume;
        }
    }
    (dp[capacity], selected)
}


/// 最小编辑距离 (MED)
/// 
/// # 参数
/// - `s`: 原始串
/// - `t`: 目标串
/// # 返回值
/// `u32`: 最小操作次数
/// `Vec<String>`: 操作步骤
/// # 备注
/// 操作优先级 替换 > 删除 > 添加
#[derive(Debug, Clone, Copy)]
enum PathMED {
    LU, //  用t[j-1]替换s[i-1], 或无操作
    U,  //  删除s[i-1]
    L,  //  添加t[j-1]
}
use PathMED::*;

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

///  钢条切割问题
/// # 参数
/// - `p`: 钢条价格表，包含钢条长度
/// # 返回值
/// `usize`: 最大收益
/// `Vec<usize>`: 切割方案
pub fn rod_cutting(p: &[u32]) -> (u32, Vec<usize>) {
    let len = p.len();
    if len == 0 {
        return (0, vec![]);
    }

    let mut dp = vec![0; len];
    let mut path = vec![0; len];

    let mut max = 0;
    let mut cut_pos = 0;
    for i in 1..=len - 1 {
        for j in 1..=i {
            if dp[i - j] + p[j] > max {
                max = dp[i - j] + p[j];
                cut_pos = j;
            }
        }
        dp[i] = max;
        path[i] = cut_pos;
    }
    //  回溯
    let mut temp_len = len - 1;
    let mut scheme = vec![];
    while temp_len > 0 {
        scheme.push(path[temp_len]);
        temp_len -= path[temp_len];
    }
    (dp[len - 1], scheme)
}