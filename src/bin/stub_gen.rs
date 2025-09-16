use pyo3_stub_gen::Result;
use rust_py_ffi::rust_to_py::stub_info;

fn main() -> Result<()> {
    // `stub_info` is a function defined by `define_stub_info_gatherer!` macro.
    stub_info()?.generate()?;
    println!("Stub files (*.pyi) generated successfully in the crate root");
    Ok(())
}
