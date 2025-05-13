import ctypes

# Load the shared library
lib = ctypes.CDLL('./target/release/libmain.so')  # Change the path as needed

# Define the return type and argument types for the `add` function
lib.add.restype = ctypes.c_int
lib.add.argtypes = [ctypes.c_int, ctypes.c_int]

# Call the function
result = lib.add(2, 3)

def add(arg1, arg2) :
    return lib.add(int(arg1), int(arg2))

print("Result of add:", result)  # Should print 5
print(result)  # Should print 5
