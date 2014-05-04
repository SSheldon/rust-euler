extern crate collections;

use std::hash::Hash;
use std::iter::count;
use collections::hashmap::HashMap;

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

fn main() {
	let min = count(1, 1).find(|&x| range(1, 21).all(|y| x % y == 0)).unwrap();
	println!("{}", min);
}
