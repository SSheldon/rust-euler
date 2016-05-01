#![feature(iter_arith)]

extern crate euler;

use std::collections::HashMap;
use std::hash::Hash;
use euler::factorization;

fn freq_count<T>(itr: T) -> HashMap<T::Item, u32>
		where T: Iterator, T::Item: Eq + Hash {
	let mut map = HashMap::new();
	for item in itr {
		*map.entry(item).or_insert(1) += 1;
	}
	map
}

fn num_factors(n: u32) -> u32 {
	let factor_count = freq_count(factorization(n));
	factor_count.values().map(|&x| x + 1).product()
}

fn main() {
	let mut triangle_numbers = (1..).map(|x| (x * (x + 1)) / 2);
	let num = triangle_numbers.find(|&x| num_factors(x) > 500).unwrap();
	println!("{}", num);
}
