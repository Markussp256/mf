pub fn get_divisors(n: usize) -> Vec<usize> {
    let mut lower_divisors = Vec::new();
    let mut upper_divisors = Vec::new();

    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            lower_divisors.push(i);
            if i != n / i {
                upper_divisors.push(n / i);
            }
        }
    }

    // Reverse the upper divisors to get them in ascending order
    upper_divisors.reverse();
    
    // Combine the two vectors
    lower_divisors.extend(upper_divisors);
    lower_divisors
}

#[test]
fn test_div() {
    assert_eq!(get_divisors(12),vec![1,2,3,4,6,12]);
    assert_eq!(get_divisors(17),vec![1,17]);
    assert_eq!(get_divisors(42),vec![1,2,3,6,7,14,21,42]);
}