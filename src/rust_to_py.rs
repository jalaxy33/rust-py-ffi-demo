use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};

/// Formats the sum of two numbers as string.
#[gen_stub_pyfunction]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Simple addition function
#[gen_stub_pyfunction]
#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

/// Calculate string length
#[gen_stub_pyfunction]
#[pyfunction]
fn string_length(s: &str) -> PyResult<usize> {
    Ok(s.len())
}

/// Complex data processing function
#[gen_stub_pyfunction]
#[pyfunction]
fn process_numbers(numbers: Vec<f64>) -> PyResult<Vec<f64>> {
    let result: Vec<f64> = numbers.iter().map(|&x| x * 2.0 + 1.0).collect();
    Ok(result)
}

/// Example using struct
#[gen_stub_pyclass]
#[pyclass]
struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

#[gen_stub_pymethods]
#[pymethods]
impl Calculator {
    #[new]
    fn new(value: f64) -> Self {
        Calculator { value }
    }

    fn add(&mut self, other: f64) -> f64 {
        self.value += other;
        self.value
    }

    fn multiply(&mut self, other: f64) -> f64 {
        self.value *= other;
        self.value
    }

    fn reset(&mut self) {
        self.value = 0.0;
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Calculator(value={})", self.value))
    }
}

/// Error handling example
#[gen_stub_pyfunction]
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(PyErr::new::<pyo3::exceptions::PyZeroDivisionError, _>(
            "Cannot divide by zero",
        ))
    } else {
        Ok(a / b)
    }
}

/// A Python module implemented in Rust.
#[pymodule(gil_used = false)]
fn rust_py_ffi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(string_length, m)?)?;
    m.add_function(wrap_pyfunction!(process_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_class::<Calculator>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
