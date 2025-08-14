use std::ops::{Add, Mul};
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() { 0 } else { self.0[0].len() }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {
            self.0[n].clone()
        } else {
            vec![]
        }
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut column = Vec::new();
        for row in &self.0 {
            if n < row.len() {
                column.push(row[n]);
            } else {
                return vec![];
            }
        }
        column
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        let rowa = self.number_of_rows();
        let colsa = self.number_of_cols();
        let rowb = other.number_of_rows();
        let colsb = other.number_of_cols();
        let mut res = vec![vec![T::default(); colsb]; rowa];
        if colsa != rowb {
            return None;
        }
        for i in 0..rowa {
            for j in 0..colsb {
                let mut sum = T::default();
                for k in 0..colsa {
                    sum = sum + self.0[i][k] * other.0[k][j];
                }
                res[i][j] = sum;
            }
        }
        Some(Matrix(res))
    }
}