use std::iter::AdditiveIterator;
use euler::Primes;

mod euler;

fn main() {
	let sum = Primes::new(2000000).sum();
	println!("{}", sum);
}
