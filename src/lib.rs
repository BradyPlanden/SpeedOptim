use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Rust Gradient function
fn rust_gradient(x: f64) -> f64 {
    (x+3.0).powf(3.0)/10.0
}

// Python-grad
pub fn gradient_descent<F>(mut x: f64, gradient: F, learn_rate: f64, epochs: u32) -> f64
where
    F: Fn(f64) -> f64,
{
    for _ in 0..epochs {
        x -= learn_rate * gradient(x);
    }
    x
}

// Rust-grad
pub fn gradient_descent_rustgrad(mut x: f64, rust_gradient: fn(f64) -> f64, learn_rate: f64, epochs: u32) -> f64 {
    for _ in 0..epochs {
        x -= learn_rate * rust_gradient(x);
    }
    x
}

#[pyfunction]
fn gradient_descent_py(
    x: f64,
    gradient: PyObject,
    learn_rate: f64,
    epochs: u32,
    py: Python,
) -> PyResult<f64> {
    let gradient = |x: f64| -> f64 {
        let args = (x,);
        gradient.call1(py, args).unwrap().extract(py).unwrap()
    };
    Ok(gradient_descent(x, gradient, learn_rate, epochs))
}

#[pyfunction]
fn gradient_descent_rust(x: f64, learn_rate: f64, epochs: u32) -> f64 {
    gradient_descent_rustgrad(x, rust_gradient, learn_rate, epochs)
}

#[pymodule]
fn speedoptim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(gradient_descent_py))?;
    m.add_wrapped(wrap_pyfunction!(gradient_descent_rust))?;
    Ok(())
}