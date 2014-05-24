extern crate collections;
extern crate num;

use collections::hashmap::HashSet;
use num::bigint::ToBigUint;

fn main() {
	let mut pows = HashSet::new();
	for a in range(2u, 101u).map(|a| a.to_biguint().unwrap()) {
		let mut pow = a.clone();
		for _ in range(2u, 101u) {
			pow = pow * a;
			// BigUint isn't hashable, so use a string instead
			pows.insert(pow.to_str());
		}
	}
	println!("{}", pows.len());
}
