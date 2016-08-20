extern crate euler;

use std::iter::repeat;
use euler::is_prime;

fn num_primes(a: i32, b: i32) -> usize {
	(1..).map(|n| n * n + a * n + b)
	     .position(|n| n <= 1 || !is_prime(n as usize))
	     .unwrap()
}

fn main() {
	let (a, b) = (-999..1000)
		.flat_map(|a| repeat(a).zip(-999..1000))
		.max_by_key(|&(a, b)| num_primes(a, b))
		.unwrap();
	println!("{}", a * b);
}
