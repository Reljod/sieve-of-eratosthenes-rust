#[macro_use]
extern crate bencher;
use algorithm::get_all_primes;

use bencher::Bencher;

fn bench_get_all_primes(b: &mut Bencher) {
    b.iter(|| get_all_primes(1000));
}

benchmark_group!(benches, bench_get_all_primes);
benchmark_main!(benches);