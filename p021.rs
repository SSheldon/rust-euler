extern crate collections;

use std::iter::AdditiveIterator;
use collections::hashmap::HashSet;
use euler::factorization;

mod euler;

fn divisors(n: uint) -> HashSet<uint> {
	let mut divisors = HashSet::new();
	divisors.insert(1u);
	for factor in factorization(n) {
		let new_divisors: Vec<uint> = divisors.iter().map(|&x| x * factor).collect();
		divisors.extend(new_divisors.move_iter());
	}
	divisors
}

fn proper_divisors(n: uint) -> HashSet<uint> {
	let mut proper_divisors = divisors(n);
	proper_divisors.remove(&n);
	proper_divisors
}

fn is_amicable(n: uint) -> bool {
	let other = proper_divisors(n).move_iter().sum();
	other != n && n == proper_divisors(other).move_iter().sum()
}

fn main() {
	let amicable_sum = range(2u, 10000u).filter(|&x| is_amicable(x)).sum();
	println!("{}", amicable_sum);
}
