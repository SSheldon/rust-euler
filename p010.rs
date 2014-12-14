extern crate euler;

use std::iter::AdditiveIterator;
use euler::Primes;

fn main() {
	let sum = Primes::new(2000000).sum();
	println!("{}", sum);
}
