extern crate euler;

use std::num::{log10, pow};
use euler::primes;

struct Rotations {
	rotation: uint,
	rot_index: uint,
	num_digits: uint,
}

impl Rotations {
	fn new(n: uint) -> Rotations {
		let num_digits = (log10(n as f64) as uint) + 1;
		Rotations{rotation: n, rot_index: 0, num_digits: num_digits}
	}
}

impl Iterator<uint> for Rotations {
	fn next(&mut self) -> Option<uint> {
		if self.rot_index < self.num_digits {
			let current = self.rotation;
			let (rem, digit) = (current / 10, current % 10);
			self.rotation = digit * pow(10u, self.num_digits - 1) + rem;
			self.rot_index += 1;
			Some(current)
		} else {
			None
		}
	}
}

fn main() {
	let prime_set = primes(1000000);
	let count = prime_set.iter()
		.count(|x| Rotations::new(x).all(|y| prime_set.contains(&y)));
	println!("{}", count);
}
