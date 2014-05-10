extern crate collections;

use std::hash::Hash;
use std::iter::{
	count,
	MultiplicativeIterator,
	range_inclusive,
};
use std::num::sqrt;
use collections::hashmap::HashMap;

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

fn freq_count<A: TotalEq + Hash, T: Iterator<A>>(mut itr: T) -> HashMap<A, uint> {
	let mut map = HashMap::new();
	for item in itr {
		map.insert_or_update_with(item, 1, |_, count| *count += 1);
	}
	map
}

fn num_factors(n: uint) -> uint {
	let factor_count = freq_count(factorization(n));
	factor_count.values().map(|&x| x + 1).product()
}

fn main() {
	let mut triangle_numbers = count(1u, 1).map(|x| (x * (x + 1)) / 2);
	let num = triangle_numbers.find(|&x| num_factors(x) > 500).unwrap();
	println!("{}", num);
}
