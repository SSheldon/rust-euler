extern crate collections;

use std::iter::{
	AdditiveIterator,
	range_inclusive,
};
use std::num::sqrt;
use collections::hashmap::HashSet;

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

fn factorization(n: uint) -> Factorization {
	Factorization{remainder: n}
}

fn divisors(n: uint) -> HashSet<uint> {
	let mut divisors = HashSet::new();
	divisors.insert(1u);
	for factor in factorization(n) {
		let new_divisors: Vec<uint> = divisors.iter().map(|&x| x * factor).collect();
		divisors.extend(new_divisors.move_iter());
	}
	divisors
}

fn proper_divisors(n: uint) -> HashSet<uint> {
	let mut proper_divisors = divisors(n);
	proper_divisors.remove(&n);
	proper_divisors
}

fn main() {
	let mut sum = 0;
	let mut abundant_nums = HashSet::new();
	for i in range_inclusive(1u, 28123u) {
		if abundant_nums.iter().all(|&x| !abundant_nums.contains(&(i - x))) {
			sum += i;
		}
		if proper_divisors(i).move_iter().sum() > i {
			abundant_nums.insert(i);
		}
	}
	println!("{}", sum);
}
