
mod factorial;
mod fibonacci;

use factorial::calculate_factorial_with_loop;
use factorial::calculate_factorial_recursively;
use fibonacci::calculate_fibonacci_with_loop;
use fibonacci::calculate_fibonacci_recursively;


#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b 
}

#[no_mangle]
pub extern "C" fn function_calculate_factorial_with_loop(a:i64)-> i64 {
    calculate_factorial_with_loop(a)
}


#[no_mangle]
pub extern "C" fn function_calculate_factorial_recursively(a: i64) -> i64 { 
    calculate_factorial_recursively(a)
}


#[no_mangle]
pub extern "C" fn function_calculate_fibonacci_with_loop(a: i64) -> i64 {
    calculate_fibonacci_with_loop(a)
}

#[no_mangle]
pub extern "C" fn function_calculate_fibonacci_recursively(a: i64) -> i64 {
    calculate_fibonacci_recursively(a)
}