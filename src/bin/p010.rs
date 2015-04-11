#![feature(core)]

extern crate euler;

use euler::Primes;

fn main() {
	let sum: usize = Primes::new(2000000).sum();
	println!("{}", sum);
}
