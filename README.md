# Rust-py-ffi-demo

A demo repo for demostrating rust-python interoperation.

## Prerequisites
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [uv package manager](https://docs.astral.sh/uv/getting-started/installation/) (auto-manages Python environment)


## Usage

**Install dependencies**:
```bash
# Install maturin
uv tool install maturin

# Install python dependencies
uv sync
```

### Rust-to-Python


```bash
# Build Rust lib
uv tool run maturin build

# Run python scripts
uv run {path/to/script}.py
# e.g. uv run src/python/call_rust.py

```

### Python-to-Rust

```bash
# build and run demo binary
cargo r --bin py_to_rust

```







