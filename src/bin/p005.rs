#![feature(core)]

extern crate euler;
extern crate num;

use std::collections::hash_map::{HashMap, Entry};
use std::hash::Hash;
use euler::factorization;
use num::{pow, range_inclusive};

fn freq_count<A, T>(itr: T) -> HashMap<A, usize>
		where A: Eq + Hash, T: Iterator<Item=A> {
	let mut map = HashMap::new();
	for item in itr {
		*map.entry(item).or_insert(0) += 1;
	}
	map
}

fn max_values<K, V, T>(itr: T) -> HashMap<K, V>
		where K: Eq + Hash, V: Ord, T: Iterator<Item=HashMap<K, V>> {
	let mut max: HashMap<K, V> = HashMap::new();
	for map in itr {
		for (key, value) in map {
			match max.entry(key) {
				Entry::Occupied(mut entry) => {
					let existing = entry.get_mut();
					if *existing < value {
						*existing = value;
					}
				},
				Entry::Vacant(entry) => {
					entry.insert(value);
				},
			}
		}
	}
	max
}

fn lcm<T>(itr: T) -> usize where T: Iterator<Item=usize> {
	let freqs = itr.map(factorization).map(freq_count);
	let factors = max_values(freqs);
	factors.into_iter().map(|(k, v)| pow(k, v)).product()
}

fn main() {
	let min = lcm(range_inclusive(1, 20));
	println!("{}", min);
}
