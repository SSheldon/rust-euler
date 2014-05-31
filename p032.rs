extern crate collections;

use std::iter::AdditiveIterator;
use collections::hashmap::HashSet;

fn from_digits(digits: &[uint]) -> uint {
	digits.iter().fold(0, |n, &d| n * 10 + d)
}

fn pandigital_product(perm: &[uint]) -> Option<uint> {
	// Since we have 9 digits total, product must be 4 digits long:
	// 4 digits for the multiplication cannot result in a 5 digit product,
	// 6 digits for the multiplication cannot result in a 3 digit product, etc
	let product_start = perm.len() - 4;
	let product = from_digits(perm.slice_from(product_start));
	for multiplier_start in range(1, product_start) {
		let multiplicand = from_digits(perm.slice_to(multiplier_start));
		let multiplier = from_digits(perm.slice(multiplier_start, product_start));
		if multiplicand * multiplier == product {
			return Some(product)
		}
	}
	None
}

fn main() {
	let digits: Vec<uint> = range(1u, 10u).collect();
	// Represent equations as a permutation of the digits 1-9
	let products: HashSet<uint> = digits.as_slice().permutations()
		.filter_map(|perm| pandigital_product(perm))
		.collect();
	let sum = products.move_iter().sum();
	println!("{}", sum);
}
