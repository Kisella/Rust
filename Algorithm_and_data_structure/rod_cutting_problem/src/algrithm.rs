/// 最小编辑距离 (MED)
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
