#[derive(Debug,Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use lalgebra_scalar::*;
impl<T> Matrix<T>
where
    T: Scalar<Item = T> + Clone,{

	pub fn new() -> Matrix<T> {
		Self(vec![vec![]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
		Self(vec![vec![T::zero();col];row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
		let mut mat = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            mat[i][i] = T::one();
        }
        Self(mat)
	}
}