use std::iter::OrdIterator;
use euler::factorization;

mod euler;

fn main() {
	let largest_factor = factorization(600851475143).max().unwrap();
	println!("{}", largest_factor);
}
