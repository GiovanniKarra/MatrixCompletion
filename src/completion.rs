use pyo3::prelude::*;
use pyo3::ffi::c_str;
use super::FMatrix;


pub fn complete_matrix(matrix: FMatrix, samples: Option<Vec<(usize, usize)>>) 
	-> FMatrix
{
	let mut completed_matrix = matrix.clone();
	let (m, n) = matrix.size();
	Python::with_gil(|py|  -> PyResult<()> {
		let code = c_str!(include_str!("completion.py"));
		let module = PyModule::from_code(
			py, code, c_str!("completion.py"), c_str!("completion"))?;

		let py_complete_matrix: Py<PyAny> = module
			.getattr("complete_matrix_ext")?
			.into();

		let args = (m, n, matrix.to_vec(), samples);
		let py_res = py_complete_matrix.call1(py,  args)?;
		let resulting_data: Vec<f32> = py_res.extract(py)?;

		completed_matrix.set_data(resulting_data);

		Ok(())
	}).expect("Python error");

	completed_matrix
}