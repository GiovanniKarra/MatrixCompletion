use std::fmt;
use std::fmt::{Display, Debug};
use std::ops::Add;

use num::Num;

pub type FMatrix = Matrix<f32>;

#[derive(Debug, Clone)]
pub struct Matrix<T: Num + Copy> {
	m: usize,
	n: usize,
	data: Vec<T>
}

impl<T: Num + Copy> Matrix<T> {
	pub fn empty(m: usize, n: usize) -> Matrix<T> {
		let data = vec![T::zero(); m*n];
		Matrix {
			m,
			n,
			data
		}
	}
	pub fn new(m: usize, n: usize, data: Vec<T>) -> Matrix<T> {
		assert_eq!(data.len(), m*n);
		Matrix {
			m,
			n,
			data
		}
	}
	pub fn get(&self, i: usize, j: usize) -> &T {
		assert!(i < self.m && j < self.n, "Index out of bound");
		self.data.get(self.n*i + j).expect("I hope this never shows up")
	}
	pub fn set(&mut self, i: usize, j: usize, val: T) {
		assert!(i < self.m && j < self.n, "Index out of bound");
		self.data[i*self.n + j] = val;
	}
	pub fn set_data(&mut self, new_data: Vec<T>) {
		assert!(new_data.len() == self.m*self.n, "Size change not allowed");
		self.data = new_data;
	}
	pub fn size(&self) -> (usize, usize) {
		(self.m, self.n)
	}
	pub fn to_vec(&self) -> Vec<T> {
		self.data.clone()
	}
}

impl<T: Num + Copy> PartialEq for Matrix<T> {
	fn eq(&self, other: &Matrix<T>) -> bool {
		self.m == other.m
		&& self.n == other.n
		&& self.data == other.data
	}
}

impl<T: Num + Copy> Add for Matrix<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		assert!(self.m == rhs.m && self.n == rhs.n,
			"Matrix sum error: size ({}, {}) is different from ({}, {})",
			self.m, self.n, rhs.m, rhs.n);

		let data = self.data
			.into_iter()
			.zip(rhs.data.into_iter())
			.map(|(a, b)| a+b)
			.collect();
		Matrix {
			m: self.m,
			n: self.n,
			data
		}
	}
}

impl<T: Num + Copy + Debug> Display for Matrix<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.m {
			write!(f, "[", )?;
			for j in 0..self.n {
				write!(f, "\t{:?}", self.get(i, j))?;	
			}
			write!(f, "\t]\n", )?;
		};
		Ok(())
    }
}