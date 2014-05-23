extern crate collections;
extern crate num;

use std::num::pow;
use collections::hashmap::HashSet;
use num::bigint::ToBigUint;

fn main() {
	let mut pows = HashSet::new();
	for a in range(2u, 101u).map(|a| a.to_biguint().unwrap()) {
		for b in range(2u, 101u) {
			// BigUint isn't hashable, so use a string instead
			pows.insert(pow(a.clone(), b).to_str());
		}
	}
	println!("{}", pows.len());
}
