import ctypes

# Load the shared library
lib = ctypes.CDLL('./target/release/libmain.so')  # Change the path as needed

# Define the return type and argument types for the `add` function
lib.add.restype = ctypes.c_int
lib.add.argtypes = [ctypes.c_int, ctypes.c_int]

def function_calculate_factorial_with_loop(arg1) :
    return lib.function_calculate_factorial_with_loop(int(arg1))

def function_calculate_factorial_recursively(arg1) :
    return lib.function_calculate_factorial_recursively(int(arg1))

def function_calculate_fibonacci_with_loop(arg1) :
    return lib.function_calculate_fibonacci_with_loop(int(arg1))

def function_calculate_fibonacci_recursively(arg1) :
    return lib.function_calculate_fibonacci_recursively(int(arg1))