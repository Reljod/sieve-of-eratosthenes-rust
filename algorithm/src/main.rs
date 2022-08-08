pub mod prime;

use env_logger::Env;
use algorithm::get_all_primes;
use algorithm::get_highest_prime_number;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let primes = get_all_primes(1000);
    println!("{:?}", primes);

    let upper_limit = 1000;
    let highest_prime = get_highest_prime_number(upper_limit);
    println!("Highest primes below {}: {:?}", upper_limit, highest_prime);
}
