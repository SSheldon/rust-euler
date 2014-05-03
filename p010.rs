extern crate collections;

use std::iter::{
	AdditiveIterator,
	range_step,
};
use collections::bitv::Bitv;

struct Primes {
	current: uint,
	stop: uint,
	primes: Bitv,
}

impl Primes {
	fn new(stop: uint) -> Primes {
		let mut primes = Bitv::new(stop, true);
		primes.set(0, false);
		primes.set(1, false);
		Primes{current: 1, stop: stop, primes: primes}
	}
}

impl Iterator<uint> for Primes {
	fn next(&mut self) -> Option<uint> {
		match range(self.current + 1, self.stop).find(|&x| self.primes.get(x)) {
			None => None,
			Some(x) => {
				// Mark all multiples of x as not prime
				for i in range_step(x + x, self.stop, x) {
					self.primes.set(i, false);
				}
				self.current = x;
				Some(x)
			}
		}
	}
}

fn main() {
	let sum = Primes::new(2000000).sum();
	println!("{}", sum);
}
