pub mod model;
use model::Commodity;
use std::cmp::max;

/* 问题背景：
 *   超市允许顾客使用一个体积大小为13的背包，选择一件或多件商品带走。问，如何才能带走总价最多的商品
 *   ------------------------
 *     商品     价格     体积
 *   ------------------------
 *     啤酒     24       10
 *     汽水     2        3
 *     饼干     9        4
 *     面包     10       5
 *     牛奶     9        4
 *     葡萄     4        1
 *   ------------------------
 */

/// - 参数
/// commodity_list: &[Commodity], 商品价格表，包括商品名、价格、体积
/// capacity: usize,              背包容量
/// - 返回值
/// u32,                          能够带走商品的最大价值
/// Vec<usize>                    带走商品的索引列表
pub fn knapsack(commodity_list: &[Commodity], capacity: usize) -> (u32, Vec<usize>) {
    if capacity == 0 || commodity_list.is_empty() {
        return (0, vec![]);
    }

    // 声明并初始化动态规划数组
    let mut dp = vec![0; capacity + 1];

    // 根据递推公式，自底向上计算，得到最优解
    for item in commodity_list.iter() {
        for c in (item.volume..=capacity).rev() {
            dp[c] = max(dp[c], dp[c - item.volume] + item.price)    //  递推公式
        }
    }

    // 回溯法重建路径，得到选择结果
    let mut remaining = capacity;
    let mut selected = vec![];
    for (i, item) in commodity_list.iter().enumerate() {
        if remaining >= item.volume && dp[remaining] == dp[remaining - item.volume] + item.price {
            selected.push(i);
            remaining -= item.volume;
        }
    }
    (dp[capacity], selected)
}