#![feature(collections, core)]

extern crate num;

use std::collections::{BitVec, BitSet};
use std::mem;
use std::num::FromPrimitive;
use num::{Integer, Num, One, Zero, range, range_inclusive, range_step};

// Factorization
fn least_divisor(n: usize) -> usize {
	match range_inclusive(2, (n as f64).sqrt() as usize).find(|&x| n % x == 0) {
		Some(x) => x,
		None => n,
	}
}

pub struct Factorization {
	remainder: usize,
}

impl Iterator for Factorization {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		if self.remainder > 1 {
			let factor = least_divisor(self.remainder);
			self.remainder /= factor;
			Some(factor)
		} else {
			None
		}
	}
}

pub fn factorization(n: usize) -> Factorization {
	Factorization{remainder: n}
}

// Primes
pub fn is_prime(n: usize) -> bool {
	range_inclusive(2, (n as f64).sqrt() as usize).all(|x| n % x != 0)
}

pub struct Primes {
	current: usize,
	stop: usize,
	primes: BitVec,
}

impl Primes {
	pub fn new(stop: usize) -> Primes {
		let mut primes = BitVec::with_capacity(stop);
		primes.set_all();
		primes.set(0, false);
		primes.set(1, false);
		Primes{current: 1, stop: stop, primes: primes}
	}
}

impl Iterator for Primes {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		match range(self.current + 1, self.stop).find(|&x| self.primes[x]) {
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

pub fn primes(stop: usize) -> BitSet {
	let mut itr = Primes::new(stop);
	// Advance the iterator to the end
	for _ in &mut itr { }
	BitSet::from_bit_vec(itr.primes)
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

impl<T: Num + Clone> Iterator for Fibonacci<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		let prev = mem::replace(&mut self.previous, self.current.clone());
		let curr = mem::replace(&mut self.current, Zero::zero());
		let next = prev + curr;
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

impl<T: Integer> Iterator for Digits<T> {
	type Item = T;

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
