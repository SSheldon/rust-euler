#![allow(dead_code)]

use std::iter::{AdditiveIterator, range_inclusive};
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

// Digits
pub struct Digits<T> {
	priv remainder: T,
	priv radix: T,
}

impl<T: Num> Digits<T> {
	pub fn new(n: T) -> Digits<T> {
		let one: |int| -> T = |_| { One::one() };
		let ten: T = range(0, 10).map(one).sum(); // Lol
		Digits{remainder: n, radix: ten}
	}
}

impl<T: Num> Iterator<T> for Digits<T> {
	fn next(&mut self) -> Option<T> {
		if !self.remainder.is_zero() {
			let digit = self.remainder % self.radix;
			self.remainder = self.remainder / self.radix;
			Some(digit)
		} else {
			None
		}
	}
}
