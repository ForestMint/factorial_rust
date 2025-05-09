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

}
