use std::iter::count;
use euler::is_prime;

mod euler;

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
