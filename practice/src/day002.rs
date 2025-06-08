use std::{cmp::max, fmt::format, io::Cursor};

fn max_continuous_subarray(array: &[i32]) -> (i32, (usize, usize)) {
    if array.is_empty() {
        panic!("")
    }
    let (mut max_sum, mut cur_sum) = (array[0], array[0]);
    let (mut start, mut end, mut temp_start) = (0, 0, 0);
    for (i, &num) in array.iter().enumerate().skip(1) {
        if cur_sum > 0 {
            cur_sum += num;
        } else {
            cur_sum = num;
            temp_start = i;
        }
        if cur_sum > max_sum {
            max_sum = cur_sum;
            end = i;
            start = temp_start;
        }
    }
    (max_sum, (start, end))
}

#[derive(Debug, Clone, Copy)]
struct Commodity {
    name: &'static str,
    price: u32,
    volume: usize,
}
fn knapsack(commodity_list: &[Commodity], capacity: usize) -> (u32, Vec<usize>) {
    if capacity == 0 || commodity_list.is_empty() {
        return (0, vec![]);
    }
    let mut dp = vec![0; capacity + 1];
    for item in commodity_list {
        for c in (item.volume..=capacity).rev() {
            dp[c] = max(dp[c - item.volume] + item.price, dp[c])
        }
    }
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

fn rod_cutting(p: &[u32]) -> (u32, Vec<usize>) {
    let len = p.len();
    if len == 0 {
        return (0, vec![]);
    }
    let mut dp = vec![0; len];
    let mut path = vec![0; len];
    let mut max = 0;
    let mut cut_pos = 0;
    for i in 1..len - 1 {
        for j in 1..i {
            if dp[i - j] + p[j] > max {
                max = dp[i - j] + p[j];
                cut_pos = j;
            }
        }
        dp[i] = max;
        path[i] = cut_pos;
    }
    let mut temp_len = len - 1;
    let mut scheme = vec![];
    while temp_len > 0 {
        scheme.push(path[temp_len]);
        temp_len -= path[temp_len];
    }
    (dp[len - 1], scheme)
}

#[derive(Debug, Clone, Copy)]
enum PathMED {
    LU,
    U,
    L,
}
use PathMED::*;
fn minmum_edit_distance(s:&str, t:&str) -> (usize, Vec<String>) {
    if s.is_empty() && t.is_empty() {
        return (0, vec![]);
    }
    let s = s.chars().collect::<Vec<_>>();
    let t= t.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0;t.len()+1];s.len()+1];
    let mut path = vec![vec![LU;t.len()+1];s.len()+1];
    for i in 0..=s.len() {
        dp[i][0] =i;
        path [i][0] =U;

    }
    for j in 0..t.len() {
        dp[0][j] =j;
        path[0][j] = L;
    }
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if dp[i][j-1] +1 < dp[i-1][j]+1
            && dp[i][j-1] +1 < dp[i-1][j-1]+ (s[i-1]!=t[j-1]) as usize {
                dp[i][j] = dp[i][j-1] +1;
                path[i][j] =L;
            }else if dp[i-1][j] < dp[i-1][j-1]+ (s[i-1]!=t[j-1])as usize {
                dp[i][j]=dp[i-1][j-1];
                path[i][j]=U;
            }else {
                dp[i][j] = dp[i-1][j-1]+(s[i-1]!=t[i-1]) as usize;
                path[i][j]= LU;
            }
        }
    }
    let (mut i, mut j) = (s.len(), t.len());
    let mut opera:Vec<String> = Vec::with_capacity(dp[i][j]);
    while i>0 ||j>0 {
        match path[i][j] {
            LU => {
                i -=1;
                j-=1;
                if s[i]== t[j] {
                    continue;
                }
                opera.push(format!("{}{}{}",i, s[i], t[j]));
            }
            U => {
                i-=1;
                opera.push(format!("{}{}",i ,s[i]));
            }
            L => {
                j-=1;
                opera.push(format!("{}",t[j]));
            }
        }
    }
    opera = opera.into_iter().rev().collect();
    (dp[s.len()][t.len()], opera)

}