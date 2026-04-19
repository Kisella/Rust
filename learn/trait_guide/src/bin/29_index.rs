use std::{ops::{Index, IndexMut}};

struct Matrix {
    data: Vec<f64>,
    cols: usize,
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

fn main() {
    let mut m = Matrix {
        data: vec![1.0, 2.0, 3.0, 4.0],
        cols: 2,
    };
    assert_eq!(m[(0, 1)], 2.0);
    m[(1, 0)] = 5.0; // 修改元素
    assert_eq!(m.data, [1.0, 2.0, 5.0, 4.0]);
}