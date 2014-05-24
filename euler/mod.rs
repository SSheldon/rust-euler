#![allow(dead_code)]

use std::iter::range_inclusive;
use std::num::{Num, One, sqrt, Zero};

// Factorization
fn least_divisor(n: uint) -> uint {
	match range_inclusive(2, sqrt(n as f64) as uint).find(|&x| n % x == 0) {
		Some(x) => x,
		None => n,
	}
}

struct Factorization {
	remainder: uint,
}

impl Iterator<uint> for Factorization {
	fn next(&mut self) -> Option<uint> {
		if self.remainder > 1 {
			let factor = least_divisor(self.remainder);
			self.remainder /= factor;
			Some(factor)
		} else {
			None
		}
	}
}

pub fn factorization(n: uint) -> Factorization {
	Factorization{remainder: n}
}

// Primes
pub fn is_prime(n: uint) -> bool {
	range_inclusive(2, sqrt(n as f64) as uint).all(|x| n % x != 0)
}

// Fibonacci
pub struct Fibonacci<T> {
	priv current : T,
	priv previous : T,
}

impl<T: Num> Fibonacci<T> {
	pub fn new() -> Fibonacci<T> {
		Fibonacci{current: Zero::zero(), previous: One::one()}
	}
}

impl<T: Num + Clone> Iterator<T> for Fibonacci<T> {
	fn next(&mut self) -> Option<T> {
		let curr = self.current.clone();
		let next = curr + self.previous;
		self.previous = curr;
		self.current = next.clone();
		Some(next)
	}
}
