use rand::Rng;
use std::{cell::RefCell, cmp::Ordering::*, rc::Rc};
pub struct Matrix {
    pub row: usize,
    pub col: usize,
}

pub fn matrix_chain_mutiply(m: &[Matrix]) -> (usize, String) {
    let n = m.len();
    let mut dp = vec![vec![0; n]; n];
    let mut path = vec![vec![0; n]; n];
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = len + i - 1;
            let mut min = usize::MAX;
            let mut cut = i;
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
    (dp[0][n - 1], get_matrix_chain_cuttiny(&path, 0, 0))
}

fn get_matrix_chain_cuttiny(path: &[Vec<usize>], left: usize, right: usize) -> String {
    if left + 1 == right {
        return format!("U_{left}U_{right}");
    }
    if left == right {
        return format!("U_{left}");
    }
    let cut = path[left][right];
    format!(
        "({})({})",
        get_matrix_chain_cuttiny(path, left, cut),
        get_matrix_chain_cuttiny(path, cut + 1, right)
    )
}

#[derive(Debug, Clone, Copy)]
enum PathLCS {
    U,
    L,
    LU,
}
use PathLCS::*;
use rand::rng;

pub fn longest_common_subsequence<T: PartialOrd + Clone>(
    x: &[T],
    y: &[T],
) -> Result<Vec<T>, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("");
    }
    let mut dp = vec![vec![0; y.len() + 1]; x.len() + 1];
    let mut path = vec![vec![U; y.len() + 1]; x.len() + 1];
    for i in 1..x.len() {
        for j in 1..y.len() {
            if x[i - 1] == y[i - 1] {
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
    let (mut i, mut j) = (x.len(), y.len());
    let mut indices = Vec::with_capacity(dp[i][j]);
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
    Ok(common_sub)
}

pub fn minmun_edit_distance(s: &str, t: &str) -> (usize, Vec<String>) {
    if s.is_empty() && t.is_empty() {
        return (0, vec![]);
    }
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut path = vec![vec![LU; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        dp[i][0] = i;
        path[i][0] = U;
    }
    for j in 0..=t.len() {
        dp[0][j] = j;
        path[0][j] = L;
    }
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if dp[i][j - 1] + 1 < dp[i - 1][j] + 1
                && dp[i][j - 1] + 1 < dp[i - 1][j - 1] + (s[i - 1] != t[i - 1]) as usize
            {
                dp[i][j] = dp[i][j - 1];
                path[i][j] = L;
            } else if dp[i - 1][j] + 1 < dp[i - 1][j - 1] + (s[i - 1] != t[i - 1]) as usize {
                dp[i][j] = dp[i - 1][j];
                path[i][j] = U;
            } else {
                dp[i][j] = dp[i - 1][j - 1] + (s[i - 1] != t[j - 1]) as usize;
                path[i][j] = LU;
            }
        }
    }
    let (mut i, mut j) = (s.len(), t.len());
    let mut opera: Vec<String> = Vec::with_capacity(dp[i][j]);
    while i > 0 || j > 0 {
        match path[i][j] {
            LU => {
                i -= 1;
                j -= 1;
                if s[i] == t[i] {
                    continue;
                }
                opera.push(format!("{} {} {}", i, s[i], t[j]));
            }
            U => {
                i -= 1;
                opera.push(format!("{} {}", i, s[i]));
            }
            L => {
                j -= 1;
                opera.push(format!("{}", t[j]));
            }
        }
    }
    opera = opera.into_iter().rev().collect();
    (dp[s.len()][t.len()], opera)
}

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
            dp[i] = max;
            path[i] = cut_pos;
        }
    }
    let mut temp_len = len - 1;
    let mut scheme = vec![];
    while temp_len > 0 {
        scheme.push(path[temp_len]);
        temp_len -= path[temp_len];
    }
    (dp[len - 1], scheme)
}

pub fn select<T: PartialOrd + PartialEq + Clone>(
    array: &mut [T],
    order: usize,
) -> Result<&T, &'static str> {
    if order < 1 || order > array.len() {
        return Err("");
    }
    let mut rng = rng();
    let mut arr = array;
    let mut target = order - 1;

    loop {
        let arr_len = arr.len();
        if arr_len == 0 {
            break Err("");
        }
        let p_idx = rng.random_range(0..arr_len);
        arr.swap(0, p_idx);
        let p = arr[0].clone();

        let (mut i, mut j) = (1, arr_len - 1);
        while i <= j {
            while i <= j && arr[i] <= p {
                i += 1;
            }
            while i < j && arr[j] > p {
                j -= 1;
            }
            if i <= j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        arr.swap(0, j);
        match target.cmp(&j) {
            Less => arr = &mut arr[..=j - 1],
            Greater => {
                arr = &mut arr[j + 1..];
                target -= j + 1;
            }
            Equal => break Ok(&arr[j]),
        }
    }
}
pub fn max_continous_subarray(array: &[i32]) -> (i32, (usize, usize)) {
    if array.is_empty() {
        panic!("");
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

pub struct List<T> {
    pub head:Option<Rc<RefCell<Node<T>>>>,
}
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T:Clone> Node<T> {
    pub fn new(value:T) -> Option<Rc<RefCell<Node<T>>>> {
        Some(Rc::new(RefCell::new(Node {value, next: None})))
    }
    pub fn get(curr:&Option<Rc<RefCell<Node<T>>>>) -> T {
        (*curr.as_ref().expect("Error: Node::get call on None!").borrow()).value.clone()
    }
    pub fn next(curr:&Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        (*curr.as_ref().expect("Error: Node::next call on None!").borrow()).next.clone()
    }
    pub fn last(head:&Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        if head.is_none() {
            panic!("Error: Node::last call on None!");

        }
        let mut curr = head.clone();
        while Node::next(&curr).is_some() {
            curr = Node::next(&curr);
        }
        curr
    }
}
impl<T:Clone + PartialEq> List<T>  {
    pub fn append(&mut self, value:T) {
        let new_node = Node::new(value);
        if self.head.is_none() {
            return (*self).head = new_node;
        }
        let last = Node::last(&self.head);
        (*last.as_ref().unwrap().borrow_mut()).next = new_node;
    }

    pub fn index(&self, index:usize) -> Result<Option<Rc<RefCell<Node<T>>>>, &'static str> {
        let mut curr = self.head.clone();
        let mut i = 0;
        while curr.is_some() && i< index {
            curr = Node::next(&curr);
            i +=1;
        }
        match i == index {
            true => Ok(curr),
            false => Err("List::index call error as index out of bounds!"),
        }
    }
    pub fn delete(&mut self, index:usize) {
        if index == 0 { 
            return (*self).head = Node::next(&self.head);
        }
        let prev = self.index(index-1).unwrap();
        let curr = Node::next(&prev);
        let next = Node::next(&curr);
        (*prev.unwrap().borrow_mut()).next = next
    }

    pub fn search(&self , value:T) -> Result<Option<Rc<RefCell<Node<T>>>>, &'static str> {
        let mut curr = self.head.clone();
        while curr.is_some() {
            if Node::get(&curr) == value {
                return Ok(curr);
            }
            curr = Node::next(&curr);
        }
        Err("The value is not found in the list")
    }
}
