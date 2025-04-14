pub struct Matrix {
    pub row: usize,
    pub col: usize,
}
impl Matrix {
    pub fn new(row: usize, col: usize) -> Matrix {
        Matrix { row, col }
    }
    pub fn new_chain(m: &[(usize, usize)]) -> Vec<Matrix> {
        let mut matriex_chain: Vec<Matrix> = Vec::with_capacity(m.len());
        for i in 0..m.len() {
            matriex_chain.push(Matrix {
                row: m[i].0,
                col: m[i].1,
            });
        }
        matriex_chain
    }
}

/*
 * 矩阵链乘法问题
 * 参数:
 * - `matrixes` 矩阵链: 相乘的各个矩阵组成的数组
 * 返回值:
 * - usize 计算矩阵链所需标量乘法的最小值
 * - String 矩阵链乘法的最优分割
 */
pub fn matrix_chain_multiply(m: &[Matrix]) -> (usize, String) {
    // 初始化动态规划数组和追踪数组
    let n = m.len();
    let mut dp = vec![vec![0; n]; n];
    let mut path = vec![vec![0; n]; n];

    // 区间动态规划
    /* 第一层循环，矩阵链长len = j-i+1, 从2增长到n */
    for len in 2..=n {
        /* 第二层循环，动态规划，计算出同一链长度下的dp值 */
        for i in 0..=(n - len) {
            let j = len + i - 1;
            let mut min: usize = usize::MAX;
            let mut cut = i;
            /* 第三层循环，寻找区间动态规划的最大值, 以及分割点k */
            for k in i..j {
                let temp = dp[i][k] + dp[k + 1][j] + m[i].row * m[k].col * m[j].col;
                if temp < min {
                    min = temp;
                    cut = k;
                }
            }
            dp[i][j] = min;
            path[i][j] = cut;
        }
    }

    /* 回溯，得到矩阵链的最优切割 */

    (dp[0][n - 1], get_matrix_chain_cuting(&path, 0, n - 1))
}

pub fn get_matrix_chain_cuting(path: &[Vec<usize>], left: usize, right: usize) -> String {
    if left + 1 == right {
        return format!("U_{left}U_{right}");
    }
    if left == right {
        return format!("U_{left}");
    }
    let cut = path[left][right];
    format!(
        "({})({})",
        get_matrix_chain_cuting(path, left, cut),
        get_matrix_chain_cuting(path, cut + 1, right)
    )
}
