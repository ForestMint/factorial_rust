
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

extern crate num_cpus;

mod factorial;
mod fibonacci;

use factorial::calculate_factorial_with_loop;
use factorial::calculate_factorial_recursively;
use fibonacci::calculate_fibonacci_with_loop;
use fibonacci::calculate_fibonacci_recursively;

fn main () {

    let cpus = num_cpus::get();
    if cpus > 1 {
        println!("We are on a multicore system with {} CPUs", cpus);
    } else {
        println!("We are on a single core system");
    }

    let number_of_iterations_factorial = 10000000;
    let parameter_factorial = 20;
    let number_of_iterations_fibonacci = 1000;
    let parameter_fibonacci = 20;

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations_factorial {
        //do my stuff
        let res_1 = calculate_factorial_with_loop(parameter_factorial);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end-start);

    let start_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations_factorial {
        //do my stuff
        let res_2 = calculate_factorial_recursively(parameter_factorial);
    }
    let end_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end_2-start_2);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations_fibonacci {
        //do my stuff
        let res_1 = calculate_fibonacci_with_loop(parameter_fibonacci);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end-start);

    let start_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations_fibonacci {
        //do my stuff
        let res_2 = calculate_fibonacci_recursively(parameter_fibonacci);
    }
    let end_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end_2-start_2);

}