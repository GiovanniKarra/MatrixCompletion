#![cfg(test)]

use libmatcompl::{complete_matrix, FMatrix};


#[test]
fn small_matrix_with_samples() {
	let matrix = FMatrix::new(3, 3,
		vec![1.0, 2.0, 3.0,
				2.0, 4.0, 6.0,
				3.0, 6.0, 9.0]);

	let complete = complete_matrix(matrix,
		Some(vec![(0, 0), (1, 1), (2, 2), (0, 2), (2, 0), (1, 0)]));

	println!("{}", complete);
}

#[test]
fn small_matrix_without_samples() {
	let matrix = FMatrix::new(3, 3,
		vec![1.0, 0.0, 3.0,
				2.0, 4.0, 0.0,
				3.0, 0.0, 9.0]);

	let complete = complete_matrix(matrix, None);

	println!("{}", complete);
}