#![feature(iter_arith)]

extern crate euler;

use std::collections::HashSet;
use euler::factorization;

fn divisors(n: u32) -> HashSet<u32> {
	let mut divisors = HashSet::new();
	divisors.insert(1);
	for factor in factorization(n) {
		let new_divisors: Vec<u32> = divisors.iter().map(|&x| x * factor).collect();
		divisors.extend(new_divisors);
	}
	divisors
}

fn proper_divisors(n: u32) -> HashSet<u32> {
	let mut proper_divisors = divisors(n);
	proper_divisors.remove(&n);
	proper_divisors
}

fn is_amicable(n: u32) -> bool {
	let other = proper_divisors(n).into_iter().sum();
	other != n && n == proper_divisors(other).into_iter().sum()
}

fn main() {
	let amicable_sum: u32 = (2..10000).filter(|&x| is_amicable(x)).sum();
	println!("{}", amicable_sum);
}
