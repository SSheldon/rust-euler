extern crate num;

use std::collections::HashSet;
use num::bigint::BigUint;

fn main() {
	let mut pows = HashSet::new();
	for a in (2u32..101).map(|a| BigUint::from(a)) {
		let mut pow = a.clone();
		for _ in 2..101 {
			pow = pow * &a;
			pows.insert(pow.clone());
		}
	}
	println!("{}", pows.len());
}
