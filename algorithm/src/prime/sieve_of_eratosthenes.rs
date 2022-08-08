use std::fmt;
use log::debug;

pub fn get_all_primes(upper_limit: usize) -> Vec<i32> {
    let mut sieve_of_eratosthenes_obj = SieveOfEratosthenesImpl::new(upper_limit);
    sieve_of_eratosthenes_obj.run();

    debug!("Metadata: {sieve_of_eratosthenes_obj:?}");
    
    return sieve_of_eratosthenes_obj.get_all_primes();
}

pub fn get_highest_prime_number(upper_limit: usize) -> i32 {
    let primes = get_all_primes(upper_limit);
    return primes.last().unwrap().clone();
}

struct SieveOfEratosthenesImpl {
    unmarked_primes_vector: Vec<i32>,
    num_of_iteration: i32,
    current_number: usize,
    upper_limit: usize,
}

impl fmt::Debug for SieveOfEratosthenesImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number of iterations/loops {}, Highest Number: {}, Upper Limit: {}", self.num_of_iteration, self.current_number, self.upper_limit)
    }
}

impl SieveOfEratosthenesImpl {
    fn new(upper_limit: usize) -> Self {
        let lower_limit = 2;

        return SieveOfEratosthenesImpl {
            unmarked_primes_vector: Self::create_unmarked_vector(upper_limit, lower_limit),
            num_of_iteration: 0,
            current_number: 2,
            upper_limit,
        };
    }

    fn run(&mut self) {
        loop {
            self.mark_multiple_of_current_number();

            let lowest_unmarked_number = self.get_lowest_unmarked_number();
            if lowest_unmarked_number == None {
                break;
            }
            self.current_number = lowest_unmarked_number.unwrap() + 2;
        }
    }

    fn get_all_primes (&self) -> Vec<i32> {
        let mut primes: Vec<i32> = Vec::new();
        for i in 0..self.unmarked_primes_vector.len() {
            if self.unmarked_primes_vector[i] == 1 {
                primes.push(i as i32 + 2);
            }
        }
        primes
    }

    fn get_lowest_unmarked_number(&self) -> Option<usize> {
        let lowest_unmarked_number = self.unmarked_primes_vector.iter().position(|&x| x == -1);
        lowest_unmarked_number
    }

    fn mark_multiple_of_current_number(&mut self) {
        let mut i = self.current_number - 2;
        loop {
            if i == self.current_number - 2 {
                let value = &mut self.unmarked_primes_vector[i];
                *value = 1;
                i += self.current_number;
                continue;
            }

            if i >= self.upper_limit-2 {
                break;
            }

            let value = &mut self.unmarked_primes_vector[i];
            *value = 0;
            i += self.current_number;
            self.num_of_iteration += 1;
        }
    }

    fn create_unmarked_vector (upper_limit: usize, lower_limit: usize) -> Vec<i32> {
        let mut unmarked_vector: Vec<i32> = Vec::new();
        for _ in lower_limit..upper_limit {
            unmarked_vector.push(-1);
        }
        unmarked_vector
    }
}