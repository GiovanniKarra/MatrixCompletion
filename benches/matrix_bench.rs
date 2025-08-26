use libmatcompl::Matrix;


pub fn sum_bench() {
	let a : Matrix<f32> = Matrix::rand(10000, 10000);
	let b : Matrix<f32> = Matrix::rand(10000, 10000);
	let _ = a + b;
}

