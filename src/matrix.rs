use std::ops::{Add, AddAssign, Index, IndexMut, Mul};
use std::fmt::{Debug, Display};

use rand::{self, Rng};

use super::types::{Float, Dot};


#[derive(Clone, Debug)]
pub struct Matrix<T: Float> {
	m: usize,
	n: usize,
	data: Box<[T]>
}

// struct MatrixView<'a, T: Float> {
// 	m: usize,
// 	n: usize,
// 	mat: &'a Matrix<T>
// }


impl<T: Float> Matrix<T> {
	pub fn new<D>(m: usize, n: usize, data: D) -> Self
	where D : Into<Box<[T]>> {
		let boxed_data : Box<[T]> = data.into();
		assert!(boxed_data.len() == m*n, "Matrix size incorrect");
		Self {m, n, data: boxed_data}
	}

	pub fn from_vec(m: usize, n: usize, data: Vec<T>) -> Self {
		assert!(data.len() == m*n, "Matrix size incorrect");
		Self {m, n, data: data.into_boxed_slice()}
	}

	pub fn from_slice(m: usize, n: usize, data: &[T]) -> Self {
		assert!(data.len() == m*n, "Matrix size incorrect");
		Self {m, n, data: Box::from(data)}
	}

	pub fn empty(m: usize, n: usize) -> Self {
		Self {m, n, data: vec![T::zero(); m*n].into_boxed_slice()}
	}

	pub fn rand(m: usize, n: usize) -> Self {
		let mut rng = rand::rng();
		Self {m, n, data: vec![T::zero(); m*n]
			.iter_mut()
			.map(|_| T::from(rng.random::<f32>()).unwrap())
			.collect::<Vec<T>>()
			.into_boxed_slice()}
	}

	pub fn get(&self, row: usize, col: usize) -> T {
		assert!(row < self.m && col < self.n, "Index out of bounds");
		self.data[row*self.n + col]
	}

	pub fn set(&mut self, row: usize, col: usize, val: T) {
		assert!(row < self.m && col < self.n, "Index out of bounds");
		self.data[row*self.n + col] = val
	}

	pub fn size(&self) -> (usize, usize) {
		(self.m, self.n)
	}

	pub fn nrows(&self) -> usize {
		self.m
	}

	pub fn ncols(&self) -> usize {
		self.n
	}

	pub fn len(&self) -> usize {
		self.m*self.n
	}
}

impl<T: Float> Index<usize> for Matrix<T> {
	type Output = [T];
	fn index(&self, index: usize) -> &Self::Output {
		assert!(index < self.m, "Index out of bounds");
		let start_index = index*self.n;
		let end_index = start_index + self.n;
		&self.data[start_index..end_index]
	}
}

impl<T: Float> IndexMut<usize> for Matrix<T> {
	// type Output = [T];
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.m, "Index out of bounds");
		let start_index = index*self.n;
		let end_index = start_index + self.n;
		&mut self.data[start_index..end_index]
	}
}

impl<T: Float> PartialEq for Matrix<T> {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
		&& self.m == other.m
		&& self.n == other.n
	}
}

impl<T: Float> Matrix<T> {
	fn add_into(left: Self, right: &Self) -> Self {
		assert!(left.size() == right.size(), "Matrix dimensions don't match");
		let mut data = left.data;
		for i in 0..data.len() {
			data[i] = data[i] + right.data[i];
		}
		Self {m: left.m, n: left.n, data: data}
	}
}

impl<T: Float> Add<&Self> for Matrix<T> {
	type Output = Matrix<T>;
	fn add(self, other: &Self) -> Self::Output {
		Matrix::add_into(self, other)
	}
}

impl<T: Float> Add for Matrix<T> {
	type Output = Matrix<T>;
	fn add(self, other: Self) -> Self::Output {
		other + &self
	}
}

impl<T: Float> Add<Matrix<T>> for &Matrix<T> {
	type Output = Matrix<T>;
	fn add(self, other: Matrix<T>) -> Self::Output {
		other + self
	}
}

impl<T: Float> Add for &Matrix<T> {
	type Output = Matrix<T>;
	fn add(self, other: Self) -> Self::Output {
		self.clone() + other
	}
}

impl<T: Float> AddAssign<&Self> for Matrix<T> {
	fn add_assign(&mut self, other: &Self) {
		assert!(self.size() == other.size(), "Matrix dimensions don't match");
		for i in 0..self.data.len() {
			self.data[i] += other.data[i];
		}
	}
}

impl<T: Float> AddAssign for Matrix<T> {
	fn add_assign(&mut self, other: Self) {
		*self += &other;
	}
}

impl<T: Float> Matrix<T> {
	fn mult_into(left: &Matrix<T>, right: &Matrix<T>, out: &mut Matrix<T>) {	
		let (rows, merge_left) = left.size();
		let (merge_right, cols) = right.size();
		assert!(merge_left == merge_right, "Input matrix dimensions don't match");
		assert!(out.size() == (rows, cols), "Output matrix dimensions don't match with input");

		for i in 0..rows {
			for j in 0..cols {
				for k in 0..merge_left {
					out[i][j] += left[i][k]*right[k][j];
				}
			}
		}
	}
}

impl<T: Float> Mul for &Matrix<T> {
	type Output = Matrix<T>;
	fn mul(self, other: Self) -> Self::Output {
		let mut new = Matrix::empty(self.m, other.n);
		Matrix::mult_into(self, other, &mut new);
		new
	}
}

impl<T: Float> Mul for Matrix<T> {	
	type Output = Matrix<T>;
	fn mul(self, other: Self) -> Self::Output {
		&self*&other
	}
}

impl<T: Float> Mul<&Self> for Matrix<T> {	
	type Output = Matrix<T>;
	fn mul(self, other: &Self) -> Self::Output {
		&self*other
	}
}

impl<T: Float> Mul<Matrix<T>> for &Matrix<T> {	
	type Output = Matrix<T>;
	fn mul(self, other: Matrix<T>) -> Self::Output {
		self*&other
	}
}

// impl<T: Float> Matrix<T> {
// 	fn mult_scalar
// }

// impl<T: Float> Mul<T> for Matrix<T> {
// 	type Output = Self;
// 	fn mul(self, scalar: T) -> Self::Output {
// 		let mut data = self.data;
// 		data
// 			.iter_mut()
// 			.for_each(|x| *x = *x * scalar);
// 		Self {
// 			m: self.m,
// 			n: self.n,
// 			data: data
// 		}
// 	}
// }

impl<T: Float> Display for Matrix<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "LOOOL")?;
		Ok(())
	}
}

impl<T: Float> Dot<T> for Matrix<T> {
	fn dot(&self, other: &Self) -> T {
		assert!(self.len() == other.len(), "Dot product matrix size mismatch");
		self.data
			.iter()
			.zip(other.data.iter())
			.map(|(x, y)| (*x) * (*y))
			.sum()
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_matrix() {
		let empty_res = Matrix {
			m: 10,
			n: 20,
			data: vec![0.0 as f32; 200].into_boxed_slice()
		};
		assert_eq!(Matrix::empty(10, 20), empty_res);

		let square_res = Matrix {
			m: 3,
			n: 3,
			data: vec![1.0, 0.0, 0.0,
					   2.0, 8.0, 0.0,
					   3.0, 9.0, 7.0].into_boxed_slice()
		};
		assert_eq!(Matrix::from_vec(3, 3, vec![1.0, 0.0, 0.0, 2.0, 8.0, 0.0, 3.0, 9.0, 7.0]), square_res);
		assert_eq!(Matrix::from_slice(3, 3, &[1.0, 0.0, 0.0, 2.0, 8.0, 0.0, 3.0, 9.0, 7.0]), square_res);
		assert_eq!(Matrix::new(3, 3, &[1.0, 0.0, 0.0, 2.0, 8.0, 0.0, 3.0, 9.0, 7.0][..]), square_res);
		assert_eq!(Matrix::new(3, 3, Box::from([1.0, 0.0, 0.0, 2.0, 8.0, 0.0, 3.0, 9.0, 7.0])), square_res);
	}


	#[test]
	#[should_panic]
	fn create_panic() {
		let _ = Matrix::new(3, 3, vec![1.0, 1.0, 1.0, 1.0]);
	}

	#[test]
	fn get_and_set_valid() {
		let mut matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		
		assert_eq!(7.0, matrix.get(2, 0));
		assert_eq!(9.0, matrix.get(2, 2));
		assert_eq!(5.0, matrix.get(1, 1));

		matrix.set(2, 0, 42.0);
		matrix.set(2, 2, 69.0);
		matrix.set(1, 1, 420.0);

		assert_eq!(42.0, matrix.get(2, 0));
		assert_eq!(69.0, matrix.get(2, 2));
		assert_eq!(420.0, matrix.get(1, 1));
	}

	#[test]
	fn get_and_set_valid_index() {
		let mut matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		
		assert_eq!(7.0, matrix[2][0]);
		assert_eq!(9.0, matrix[2][2]);
		assert_eq!(5.0, matrix[1][1]);

		matrix[2][0] = 42.0;
		matrix[2][2] = 69.0;
		matrix[1][1] = 420.0;

		assert_eq!(42.0, matrix[2][0]);
		assert_eq!(69.0, matrix[2][2]);
		assert_eq!(420.0, matrix[1][1]);
	}

	#[test]
	#[should_panic]
	fn get_panic1() {
		let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		let _ = matrix.get(4, 0);
	}

	#[test]
	#[should_panic]
	fn get_panic2() {
		let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		let _ = matrix.get(0, 4);
	}

	#[test]
	#[should_panic]
	fn set_panic1() {
		let mut matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		matrix.set(4, 0, 42.0);
	}

	#[test]
	#[should_panic]
	fn set_panic2() {
		let mut matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		matrix.set(0, 4, 42.0);
	}

	#[test]
	fn sum1() {
		let m1 = Matrix::new(2, 2, vec![1.0, 1.0, 2.0, 3.0]);
		let m2 = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		let res = Matrix::new(2, 2, vec![3.0, 3.0, 3.0, 3.0]);
		assert_eq!(&m1 + &m2, res);
		assert_eq!(m1 + m2, res);
	}

	#[test]
	fn sum2() {
		let mut m1 = Matrix::new(2, 2, vec![1.0, 1.0, 2.0, 3.0]);
		let m2 = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		let res = Matrix::new(2, 2, vec![3.0, 3.0, 3.0, 3.0]);
		m1 += m2;
		assert_eq!(m1, res);
	}

	#[test]
	#[should_panic]
	fn sum_panic1() {
		let m1 = Matrix::new(2, 3, vec![1.0, 1.0, 2.0, 3.0, 0.0, 0.0]);
		let m2 = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		println!("{:?}", m1 + m2);
	}

	#[test]
	#[should_panic]
	fn sum_panic2() {
		let m1 = Matrix::new(2, 3, vec![1.0, 1.0, 2.0, 3.0, 0.0, 0.0]);
		let m2 = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		println!("{:?}", m2 + m1);
	}

	#[test]
	fn test_print() {
		let vector = Matrix::new(3, 1, vec![1.0, 2.0, 3.0]);
		println!("{}", vector);
	}

	#[test]
	fn test_index() {
		let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		for i in 0..matrix.m {
			for j in 0..matrix.n {
				assert_eq!(matrix[i][j], matrix.get(i, j));
			}
		}
		assert_eq!(matrix[2], [7.0, 8.0, 9.0]);
	}

	#[test]
	fn test_mult() {
		let left = Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
		let right = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		let res = left * &right;
		assert_eq!(res, right);

		let left = Matrix::new(1, 3, vec![1.0, 0.0, 0.0]);
		let res = left * right;
		assert_eq!(res, Matrix::new(1, 3, vec![1.0, 2.0, 3.0]));

		let right = Matrix::new(2, 3, vec![1.0, 1.0, 2.0, 3.0, 0.0, 0.0]);
		let left = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		assert_eq!(left*right, Matrix::new(2, 3, vec![8.0, 2.0, 4.0, 1.0, 1.0, 2.0]));
	}
	
	#[test]
	#[should_panic]
	fn mult_panic() {
		let left = Matrix::new(2, 3, vec![1.0, 1.0, 2.0, 3.0, 0.0, 0.0]);
		let right = Matrix::new(2, 2, vec![2.0, 2.0, 1.0, 0.0]);
		let _ = left * right;
	}

	#[test]
	fn test_dot() {
		let left = Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
		let right = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
		let res = left.dot(&right);
		assert_eq!(res, 15.0);

		let left = Matrix::new(1, 3, vec![1.0, 0.0, 0.0]);
		let right = Matrix::new(3, 1, vec![1.0, 2.0, 3.0]);
		let res = left.dot(&right);
		assert_eq!(res, 1.0);
	}

	#[test]
	#[should_panic]
	fn dot_panic() {
		let left = Matrix::new(1, 3, vec![1.0, 0.0, 0.0]);
		let right = Matrix::new(4, 1, vec![1.0, 2.0, 3.0, 4.0]);
		let _ = left.dot(&right);
	}
}
