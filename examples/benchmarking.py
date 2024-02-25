import timeit
from speedoptim import gradient_descent_py
from speedoptim import gradient_descent_rust

# Setup timing
number = 1000  # Number of times to run the function
x = 10
learn_rate = 0.001
epochs = 1000

# Define the python gradient of the function
def gradient(x):
    return ((x + 3)**3)/10

# Define the python gradient descent function
def gradient_descent_python(x, gradient, learn_rate, epochs):
    for i in range(epochs):
        x -= learn_rate * gradient(x)
    return x

# Setup timing functions
def func_rust_pygrad():
    gradient_descent_py(x, gradient, learn_rate, epochs)

def func_py():
    gradient_descent_python(x, gradient, learn_rate, epochs)

def func_rust():
    gradient_descent_rust(x, learn_rate, epochs)

# Time the functions
time_rust_pygrad = timeit.timeit(func_rust_pygrad, number=number) / number
time_py = timeit.timeit(func_py, number=number) / number
time_rust = timeit.timeit(func_rust, number=number) / number
print(f"Rust w/ python gradient: {time_rust_pygrad} seconds \nPure python: {time_py} seconds \nRust w/ rust gradient: {time_rust} seconds") 

# Output:
# Rust w/ python gradient: 8.586975000071106e-05 seconds
# Pure python: 7.247033300154726e-05 seconds
# Rust w/ rust gradient: 2.8472166999563342e-05 seconds

# Verify that the results are the same
assert gradient_descent_py(x, gradient, learn_rate, epochs) == gradient_descent_rust(x, learn_rate, epochs) == gradient_descent_python(x, gradient, learn_rate, epochs)

