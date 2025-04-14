use std::cmp::max;
use crate::model::Commodity;

// 0-1背包问题
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
