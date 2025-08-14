use std::ops::{ Add, Sub };

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T> where T: Add<Output = T> + Copy {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::new();
        for (row_a, row_b) in self.0.iter().zip(other.0.iter()) {
            if row_a.len() != row_b.len() {
                return None;
            }
            let mut new_row = Vec::new();
            for (a, b) in row_a.iter().zip(row_b.iter()) {
                new_row.push(*a + *b);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T> where T: Sub<Output = T> + Copy {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();

        for (row_a, row_b) in self.0.iter().zip(other.0.iter()) {
            if row_a.len() != row_b.len() {
                return None;
            }

            let mut new_row = Vec::new();
            for (a, b) in row_a.iter().zip(row_b.iter()) {
                new_row.push(*a - *b);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}
