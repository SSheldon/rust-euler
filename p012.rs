extern crate collections;
extern crate euler;

use std::hash::Hash;
use std::iter::{
	count,
	MultiplicativeIterator,
};
use collections::hashmap::HashMap;
use euler::factorization;

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
