#![feature(iter_arith)]

extern crate num;
extern crate euler;

use std::collections::HashSet;
use num::range_inclusive;
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

fn is_abundant(n: u32) -> bool {
	let divisor_sum: u32 = proper_divisors(n).into_iter().sum();
	divisor_sum > n
}

fn main() {
	let mut sum = 0;
	let mut abundant_nums = HashSet::new();
	for i in range_inclusive(1, 28123) {
		if abundant_nums.iter().all(|&x| !abundant_nums.contains(&(i - x))) {
			sum += i;
		}
		if is_abundant(i) {
			abundant_nums.insert(i);
		}
	}
	println!("{}", sum);
}
