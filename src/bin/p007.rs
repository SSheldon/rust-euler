extern crate euler;

use std::iter::count;
use euler::is_prime;

fn main() {
	let prime = count(2u, 1).filter(|&x| is_prime(x)).nth(10000).unwrap();
	println!("{}", prime);
}
