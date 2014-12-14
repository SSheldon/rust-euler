extern crate collections;
extern crate euler;

use std::hash::Hash;
use std::iter::{
	range_inclusive,
	MultiplicativeIterator,
};
use std::num::pow;
use collections::hashmap::HashMap;
use euler::factorization;

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

fn lcm<T: Iterator<uint>>(itr: T) -> uint {
	let factors = max_values(itr.map(|x| freq_count(factorization(x))));
	factors.iter().map(|(&k, &v)| pow(k, v)).product()
}

fn main() {
	let min = lcm(range_inclusive(1u, 20u));
	println!("{}", min);
}
