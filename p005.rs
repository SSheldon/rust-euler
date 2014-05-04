extern crate collections;

use std::hash::Hash;
use std::iter::{
	range_inclusive,
	MultiplicativeIterator,
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

fn max_values<K: TotalEq + Hash + Clone, V: Ord + Clone, T: Iterator<HashMap<K, V>>>(mut itr: T) -> HashMap<K, V> {
	let mut max: HashMap<K, V> = HashMap::new();
	for map in itr {
		for (key, value) in map.iter() {
			if max.contains_key(key) {
				let existing = max.get_mut(key);
				if existing.lt(value) {
					*existing = value.clone();
				}
			} else {
				max.insert(key.clone(), value.clone());
			}
		}
	}
	max
}

fn pow_with_uint(radix: uint, pow: uint) -> uint {
	range(0, pow).fold(1, |n, _| n * radix)
}

fn lcm<T: Iterator<uint>>(itr: T) -> uint {
	let factors = max_values(itr.map(|x| freq_count(factorization(x))));
	factors.iter().map(|(&k, &v)| pow_with_uint(k, v)).product()
}

fn main() {
	let min = lcm(range_inclusive(1u, 20u));
	println!("{}", min);
}
