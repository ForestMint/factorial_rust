
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

mod factorial;

use factorial::calculate_factorial_with_loop;
use factorial::calculate_factorial_recursively;

fn main () {

    let number_of_iterations = 10000000;
    let parameter = 20;

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations {
        //do my stuff
        let res_1 = calculate_factorial_with_loop(parameter);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end-start);

    let start_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..number_of_iterations {
        //do my stuff
        let res_2 = calculate_factorial_recursively(parameter);
    }
    let end_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end_2-start_2);


    //println!("{}", res_1);
    //println!("{}", res_2);
}