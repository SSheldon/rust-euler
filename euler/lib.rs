extern crate num;

use std::collections::bitv::Bitv;
use std::collections::bitv_set::BitvSet;
use std::iter::{range_inclusive, range_step};
use std::num::Float;
use self::num::{Integer, Num, One, Zero};

// Factorization
fn least_divisor(n: uint) -> uint {
	match range_inclusive(2, (n as f64).sqrt() as uint).find(|&x| n % x == 0) {
		Some(x) => x,
		None => n,
	}
}

#[allow(missing_copy_implementations)]
pub struct Factorization {
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
	range_inclusive(2, (n as f64).sqrt() as uint).all(|x| n % x != 0)
}

pub struct Primes {
	current: uint,
	stop: uint,
	primes: Bitv,
}

impl Primes {
	pub fn new(stop: uint) -> Primes {
		let mut primes = Bitv::with_capacity(stop, true);
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

pub fn primes(stop: uint) -> BitvSet {
	let mut itr = Primes::new(stop);
	// Advance the iterator to the end
	for _ in itr { }
	BitvSet::from_bitv(itr.primes)
}

// Fibonacci
pub struct Fibonacci<T> {
	current : T,
	previous : T,
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
	remainder: T,
	radix: T,
}

impl<T: Integer + FromPrimitive> Digits<T> {
	pub fn new(n: T) -> Digits<T> {
		let ten = FromPrimitive::from_u8(10).unwrap();
		Digits{remainder: n, radix: ten}
	}
}

impl<T: Integer> Iterator<T> for Digits<T> {
	fn next(&mut self) -> Option<T> {
		if !self.remainder.is_zero() {
			let (rem, digit) = self.remainder.div_rem(&self.radix);
			self.remainder = rem;
			Some(digit)
		} else {
			None
		}
	}
}
