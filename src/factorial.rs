pub fn calculate_factorial_with_loop (mut n: i64) -> i64 {
    let mut result = 1;
    while n>0 {
        result *= n;
        n-=1;
    }
    result
}

pub fn calculate_factorial_recursively (n: i64) -> i64 {
    if n==0 {
        1
    } else {
        n*calculate_factorial_recursively(n-1)
    }
}

pub fn calculate_fibonacci_with_loop (mut n: i64) -> i64 {
    let mut result = 1;
    let mut value_following_result = 1;
    while n>1 {
        let mem = value_following_result;
        value_following_result+=result;
        result = mem;
        n-=1;
    }
    result
}

pub fn calculate_fibonacci_recursively (n: i64)  -> i64 {
    match n {
        1 => 1,
        2 => 1,
        // Handle the rest of cases
        _ => calculate_fibonacci_recursively(n-2)+calculate_fibonacci_recursively(n-1),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_factorial_with_loop() {
        assert_eq!(calculate_factorial_with_loop(0), 1);
        assert_eq!(calculate_factorial_with_loop(1), 1);
        assert_eq!(calculate_factorial_with_loop(2), 2);
        assert_eq!(calculate_factorial_with_loop(3), 6);
        assert_eq!(calculate_factorial_with_loop(4), 24);
        assert_eq!(calculate_factorial_with_loop(5), 120);
    }

    #[test]
    fn test_calculate_factorial_recursively() {
        assert_eq!(calculate_factorial_with_loop(0), 1);
        assert_eq!(calculate_factorial_with_loop(1), 1);
        assert_eq!(calculate_factorial_with_loop(2), 2);
        assert_eq!(calculate_factorial_with_loop(3), 6);
        assert_eq!(calculate_factorial_with_loop(4), 24);
        assert_eq!(calculate_factorial_with_loop(5), 120);
    }

    #[test]
    fn test_calculate_fibonacci_recursively() {
        assert_eq!(calculate_fibonacci_recursively(1), 1);
        assert_eq!(calculate_fibonacci_recursively(2), 1);
        assert_eq!(calculate_fibonacci_recursively(3), 2);
        assert_eq!(calculate_fibonacci_recursively(4), 3);
        assert_eq!(calculate_fibonacci_recursively(5), 5);
        assert_eq!(calculate_fibonacci_recursively(6), 8);
        assert_eq!(calculate_fibonacci_recursively(7), 13);
        assert_eq!(calculate_fibonacci_recursively(8), 21);
    }

    #[test]
    fn test_calculate_fibonacci_with_loop() {
        assert_eq!(calculate_fibonacci_with_loop(1), 1);
        assert_eq!(calculate_fibonacci_with_loop(2), 1);
        assert_eq!(calculate_fibonacci_with_loop(3), 2);
        assert_eq!(calculate_fibonacci_with_loop(4), 3);
        assert_eq!(calculate_fibonacci_with_loop(5), 5);
        assert_eq!(calculate_fibonacci_with_loop(6), 8);
        assert_eq!(calculate_fibonacci_with_loop(7), 13);
        assert_eq!(calculate_fibonacci_with_loop(8), 21);
    }
}
