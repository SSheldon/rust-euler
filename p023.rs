extern crate collections;

use std::iter::{
	AdditiveIterator,
	range_inclusive,
};
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

fn main() {
	let mut sum = 0;
	let mut abundant_nums = HashSet::new();
	for i in range_inclusive(1u, 28123u) {
		if abundant_nums.iter().all(|&x| !abundant_nums.contains(&(i - x))) {
			sum += i;
		}
		if proper_divisors(i).move_iter().sum() > i {
			abundant_nums.insert(i);
		}
	}
	println!("{}", sum);
}
