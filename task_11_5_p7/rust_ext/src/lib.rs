use pyo3::prelude::*;

#[pyfunction]
pub fn sum_as_string(a: i32, b: i32) -> String {
    (a + b).to_string()
}

#[pymodule]
fn rust_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}