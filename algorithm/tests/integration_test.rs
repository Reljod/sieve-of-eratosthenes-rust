use algorithm::get_all_primes;
use algorithm::get_highest_prime_number;

#[test]
fn test_get_all_primes() {
    let primes = get_all_primes(10);
    assert_eq!(primes, vec![2, 3, 5, 7]);
}

#[test]
fn test_get_highest_prime_number() {
    let highest_prime_number = get_highest_prime_number(10);
    assert_eq!(highest_prime_number, 7);
}
