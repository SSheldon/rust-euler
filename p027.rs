use std::iter::{count, range_inclusive};
use std::num::sqrt;

fn is_prime(n: uint) -> bool {
	range_inclusive(2, sqrt(n as f64) as uint).all(|x| n % x != 0)
}

fn num_primes(a: int, b: int) -> uint {
	count(1, 1).map(|n| n * n + a * n + b)
	           .position(|n| n <= 1 || !is_prime(n as uint))
	           .unwrap()
}

fn main() {
	let mut pairs = range(0, 1999 * 1999).map(|x| (x / 1999 - 999, x % 1999 - 999));
	let (a, b) = pairs.max_by(|&(a, b)| num_primes(a, b)).unwrap();
	println!("{}", a * b);
}
