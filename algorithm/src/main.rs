use env_logger::Env;
use log::{debug,info};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let upper_limit: usize = 1000;

    info!("Upper Limit is {}", upper_limit);
    let mut nonmarked_array = create_nonmarked_array(upper_limit);

    let mut iteration = 1;
    let mut p = 2;
    loop {
        let mut i = p - 2;
        loop {
            if i == p - 2 {
                let value = &mut nonmarked_array[i];
                *value = 1;
                i += p;
                continue;
            }

            if i >= upper_limit-2 {
                break;
            }

            debug!("mark index {} or number {}", i, i + 2);
            let value = &mut nonmarked_array[i];
            *value = 0;
            i += p;
        }
        let first_unmarked_number = nonmarked_array.iter().position(|&x| x == -1);
        if first_unmarked_number == None {
            break;
        }

        p = first_unmarked_number.unwrap() + 2;
        debug!("First unmarked number {}", p);
        debug!("iteration {}", iteration);
        iteration += 1;

    }

    let highest_prime_number = nonmarked_array.iter().rposition(|&x| x == 1).unwrap() + 2;
    info!("Highest prime number is {}", highest_prime_number);
    let all_primes = get_all_primes(&nonmarked_array);
    info!("The prime numbers between 0 and {} are {:?}", upper_limit, all_primes);
}

fn create_nonmarked_array (upper_limit: usize) -> Vec<i32> {
    let mut nonmarked_array: Vec<i32> = Vec::new();
    let lower_limit: usize = 2;
    for _ in lower_limit..upper_limit {
        nonmarked_array.push(-1);
    }
    nonmarked_array
}

fn get_all_primes (marked_primes_array: &Vec<i32>) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();
    for i in 0..marked_primes_array.len() {
        if marked_primes_array[i] == 1 {
            primes.push(i as i32 + 2);
        }
    }
    primes
}
