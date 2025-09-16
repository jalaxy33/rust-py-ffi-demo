//! Reference: https://pyo3.rs/v0.26.0/python-from-rust/calling-existing-code.html
//

use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;
use std::ffi::CString;


pub fn display_py_version() -> PyResult<()> {
    println!("\n[Display Python Version]");
    Python::attach(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

pub fn import_py_lib() -> PyResult<()> {
    println!("\n[Import Python Library]");
    Python::attach(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        assert_eq!(total, 6);
        println!("1 + 2 + 3 = {}", total);
        Ok(())
    })
}

pub fn eval_py_expression() -> PyResult<()> {
    println!("\n[Eval Python Expression]");
    Python::attach(|py| {
        let result = py
            .eval(c_str!("[i * 10 for i in range(5)]"), None, None)
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap();
        let res: Vec<i64> = result.extract().unwrap();
        assert_eq!(res, vec![0, 10, 20, 30, 40]);
        println!("Eval [i * 10 for i in range(5)]: {:?}", res);
        Ok(())
    })
}

#[pyclass]
struct UserData {
    id: u32,
    name: String,
}

#[pymethods]
impl UserData {
    fn as_tuple(&self) -> (u32, String) {
        (self.id, self.name.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("User {}(id: {})", self.name, self.id))
    }
}

pub fn run_py_statement() -> PyResult<()> {
    println!("\n[Run Python Statement]");
    Python::attach(|py| {
        let userdata = UserData {
            id: 34,
            name: "Yu".to_string(),
        };
        let userdata = Py::new(py, userdata).unwrap();
        let userdata_as_tuple = (34, "Yu");
        py_run!(py, userdata userdata_as_tuple, r#"
assert repr(userdata) == "User Yu(id: 34)"
assert userdata.as_tuple() == userdata_as_tuple
print(f"User data from Rust: {userdata}")
    "#);
        Ok(())
    })
}

pub fn from_py_code_str() -> PyResult<()> {
    println!("\n[From Python Code String]");
    Python::attach(|py| {
        let activators = PyModule::from_code(
            py,
            c_str!(
                r#"
def relu(x):
    """see https://en.wikipedia.org/wiki/Rectifier_(neural_networks)"""
    return max(0.0, x)

def leaky_relu(x, slope=0.01):
    return x if x >= 0 else x * slope
    "#
            ),
            c_str!("activators.py"),
            c_str!("activators"),
        )?;

        let relu_result: f64 = activators.getattr("relu")?.call1((-1.0,))?.extract()?;
        assert_eq!(relu_result, 0.0);

        let kwargs = [("slope", 0.2)].into_py_dict(py)?;
        let lrelu_result: f64 = activators
            .getattr("leaky_relu")?
            .call((-1.0,), Some(&kwargs))?
            .extract()?;
        assert_eq!(lrelu_result, -0.2);
        println!(
            "relu(-1.0) = {}, leaky_relu(-1.0, slope=0.2) = {}",
            relu_result, lrelu_result
        );

        Ok(())
    })
}

pub fn from_py_code_file() -> PyResult<()> {
    println!("\n[From Python Code File]");

    Python::attach(|py| {
        let code_path = std::env::current_dir()?.join("src/python/math_utils.py");
        let code = std::fs::read_to_string(code_path)?;

        let math_utils = PyModule::from_code(
            py,
            CString::new(code)?.as_c_str(),
            c_str!("math_utils.py"),
            c_str!("math_utils"),
        )?;

        let add_result: i32 = math_utils
            .getattr("add_numbers")?
            .call1((3, 5))?
            .extract()?;
        assert_eq!(add_result, 8);

        let factorial_5: u64 = math_utils.getattr("factorial")?.call1((5,))?.extract()?;
        assert_eq!(factorial_5, 120);

        let fib_10: u64 = math_utils.getattr("fibonacci")?.call1((10,))?.extract()?;
        assert_eq!(fib_10, 55);

        println!(
            "3 + 5 = {} , factorial(5) = {}, fibonacci(10) = {}",
            add_result, factorial_5, fib_10
        );
        Ok(())
    })
}
