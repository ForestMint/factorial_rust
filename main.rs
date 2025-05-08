
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

fn calculate_factorial_with_loop (mut n: i64) -> i64 {
    let mut result = 1;
    while n>0 {
        result *= n;
        n-=1;
    }
    result
}

fn calculate_factorial_recursively (n: i64) -> i64 {
    if n==0 {
        1
    } else {
        n*calculate_factorial_recursively(n-1)
    }
}

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