use num::Num;
use pyo3::prelude::*;
use pyo3::ffi::c_str;
use super::Matrix;

fn complete_matrix<T: Num + Copy + IntoPyObject<'static> + FromPyObject<'static>>
	(matrix: Matrix<T>, samples: Option<Vec<(usize, usize)>>) 
	-> Matrix<T>
{
	let mut completed_matrix = matrix.clone();
	Python::with_gil(|py| -> PyResult<()> {
		let code = c_str!(include_str!("completion.py"));
		let module = PyModule::from_code(
			py, code, c_str!("completion.py"), c_str!("completion"))?;

		let py_complete_matrix: Py<PyAny> = module
			.getattr("matrix_completion_ext")?
			.into();

		let py_res = py_complete_matrix.call1(py, (matrix.to_vec(), samples))?;
		let resulting_data: Vec<T> = py_res.extract(py)?;

		completed_matrix.set_data(resulting_data);

		Ok(())
	});

	completed_matrix
}