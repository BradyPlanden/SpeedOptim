# speedoptim

`speedoptim` is a hybrid Python/Rust package designed to offer high-performance computation capabilities by leveraging Rust's speed and safety within a Python package. This package can be developed and built using `maturin`.

## Requirements

- Python 3.8+
- Rust 1.41+
- Cargo (Rust's package manager)
- maturin (`pip install maturin`)

## Development Setup

To develop `speedoptim`, you first need to ensure that Rust and Cargo are installed on your system. If not, install them using [rustup](https://rustup.rs/), which is Rust's recommended toolchain manager.

Next, install `maturin` within your Python environment using pip:

```bash
pip install maturin
```

## Building the Package

To build the package, navigate to the root directory of speedoptim and run:

```bash
maturin build
```

This command will compile the Rust code and package it into a wheel that can be installed with pip. By default, maturin build targets the active Python environment.

For a development build, which compiles faster but doesn't optimize the Rust code, you can use:

```bash
maturin develop
```

This will build and install the package directly into your current Python environment, allowing you to import and use speedoptim in Python immediately.

If you want to build the package for a different Python version or for multiple versions, you can specify the Python executables:

```bash
maturin build --interpreter python3.6 --interpreter python3.7
```

## Running Tests

To ensure the quality of speedoptim, you can run tests using maturin:

```bash
maturin test
```

This will run the Rust test suite for speedoptim.