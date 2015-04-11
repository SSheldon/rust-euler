extern crate euler;

use euler::factorization;

fn main() {
	let largest_factor = factorization(600851475143u64).max().unwrap();
	println!("{}", largest_factor);
}
