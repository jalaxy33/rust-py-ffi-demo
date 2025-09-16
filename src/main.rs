use pyo3::prelude::*;

use rust_py_ffi::from_py::*;

fn main() -> PyResult<()> {
    // Python::initialize();

    display_py_version()?;
    import_py_lib()?;
    eval_py_expression()?;
    run_py_statement()?;
    from_py_code_str()?;
    from_py_code_file()?;

    Ok(())
}
