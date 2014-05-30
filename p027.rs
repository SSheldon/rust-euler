use std::iter::{count, Repeat};
use euler::is_prime;

mod euler;

fn num_primes(a: int, b: int) -> uint {
	count(1, 1).map(|n| n * n + a * n + b)
	           .position(|n| n <= 1 || !is_prime(n as uint))
	           .unwrap()
}

fn main() {
	let (a, b) = range(-999, 1000)
		.flat_map(|a| Repeat::new(a).zip(range(-999, 1000)))
		.max_by(|&(a, b)| num_primes(a, b))
		.unwrap();
	println!("{}", a * b);
}
