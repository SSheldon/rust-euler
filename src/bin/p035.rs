extern crate num;
extern crate euler;

use num::pow;
use euler::primes;

struct Rotations {
	rotation: usize,
	rot_index: usize,
	num_digits: usize,
}

impl Rotations {
	fn new(n: usize) -> Rotations {
		let num_digits = ((n as f64).log10() as usize) + 1;
		Rotations{rotation: n, rot_index: 0, num_digits: num_digits}
	}
}

impl Iterator for Rotations {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		if self.rot_index < self.num_digits {
			let current = self.rotation;
			let (rem, digit) = (current / 10, current % 10);
			self.rotation = digit * pow(10, self.num_digits - 1) + rem;
			self.rot_index += 1;
			Some(current)
		} else {
			None
		}
	}
}

fn main() {
	let prime_set = primes(1000000);
	let count = prime_set.into_iter()
		.filter(|&x| Rotations::new(x).all(|y| prime_set.contains(y)))
		.count();
	println!("{}", count);
}
