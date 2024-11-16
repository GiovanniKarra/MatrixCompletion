#![cfg(test)]

use std::vec;

use libmatcompl::matrix::Matrix;

#[test]
fn create_matrix() {
	let empty_res = Matrix {
		m: 10,
		n: 20,
		data: vec![0.0 as f32; 200]
	};
	assert_eq!(Matrix::empty(10, 20), empty_res);

	let square_res = Matrix {
		m: 3,
		n: 3,
		data: vec![1, 0, 0,
				   2, 8, 0,
				   3, 9, 7]
	};
	assert_eq!(Matrix::new(3, 3, vec![1, 0, 0, 2, 8, 0, 3, 9, 7]), square_res);
}

#[test]
fn get_and_set_valid() {
	let mut matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	
	assert_eq!(7, *matrix.get(2, 0));
	assert_eq!(9, *matrix.get(2, 2));
	assert_eq!(5, *matrix.get(1, 1));

	matrix.set(2, 0, 42);
	matrix.set(2, 2, 69);
	matrix.set(1, 1, 420);

	assert_eq!(42, *matrix.get(2, 0));
	assert_eq!(69, *matrix.get(2, 2));
	assert_eq!(420, *matrix.get(1, 1));
}

#[test]
#[should_panic]
fn get_panic1() {
	let matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	let _ = matrix.get(4, 0);
}

#[test]
#[should_panic]
fn get_panic2() {
	let matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	let _ = matrix.get(0, 4);
}

#[test]
#[should_panic]
fn set_panic1() {
	let mut matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	matrix.set(4, 0, 42);
}

#[test]
#[should_panic]
fn set_panic2() {
	let mut matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	matrix.set(0, 4, 42);
}

#[test]
fn sum() {
	let m1 = Matrix::new(2, 2, vec![1, 1, 2, 3]);
	let m2 = Matrix::new(2, 2, vec![2, 2, 1, 0]);
	let res = Matrix::new(2, 2, vec![3, 3, 3, 3]);
	println!("{}", m1.clone() + m2.clone());
	assert_eq!(m1 + m2, res);
}

#[test]
#[should_panic]
fn sum_panic() {
	let m1 = Matrix::new(2, 3, vec![1, 1, 2, 3, 0, 0]);
	let m2 = Matrix::new(2, 2, vec![2, 2, 1, 0]);
	println!("{}", m1.clone() + m2.clone());
}

#[test]
fn test_print() {
	let vector = Matrix::new(3, 1, vec![1, 2, 3]);
	println!("{}", vector);
}