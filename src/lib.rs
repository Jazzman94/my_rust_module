use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn my_rust_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    Ok(())
}

#[pyfunction]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[pyfunction]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}