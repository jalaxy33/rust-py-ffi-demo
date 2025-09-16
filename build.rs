use std::env;
use std::path::Path;

fn main() {
    #[cfg(target_os = "windows")]
    link_windows_python_dll();
}

#[cfg(target_os = "windows")]
fn link_windows_python_dll() {
    let python_executable =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(".venv/Scripts/python.exe");
    let python_path_str = &python_executable.to_str().unwrap().to_string();

    assert!(
        python_executable.exists(),
        "Python executable not found at {}",
        python_path_str
    );

    let output = std::process::Command::new(python_path_str)
        .args(&["-c", "import sys; print(sys.base_exec_prefix)"])
        .output()
        .expect("Failed to execute python command to get executable path");
    match output.status.success() {
        true => {
            let py_dll_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_env = env::var("PATH").unwrap_or_default();
            println!("cargo:rustc-env=PATH={};{}", path_env, py_dll_dir);
        }
        false => {
            println!("cargo:warning=Failed to get Python executable path");
        }
    }

}
