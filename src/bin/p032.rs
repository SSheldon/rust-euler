extern crate euler;

use std::collections::HashSet;
use euler::permute;

fn from_digits(digits: &[u32]) -> u32 {
	digits.iter().fold(0, |n, &d| n * 10 + d)
}

fn pandigital_product(perm: &[u32]) -> Option<u32> {
	// Since we have 9 digits total, product must be 4 digits long:
	// 4 digits for the multiplication cannot result in a 5 digit product,
	// 6 digits for the multiplication cannot result in a 3 digit product, etc
	let product_start = perm.len() - 4;
	let product = from_digits(&perm[product_start..]);
	for multiplier_start in 1..product_start {
		let multiplicand = from_digits(&perm[..multiplier_start]);
		let multiplier = from_digits(&perm[multiplier_start..product_start]);
		if multiplicand * multiplier == product {
			return Some(product)
		}
	}
	None
}

fn main() {
	let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
	let mut products = HashSet::new();
	// Represent equations as a permutation of the digits 1-9
	permute(&mut digits, |perm| {
		if let Some(product) = pandigital_product(perm) {
			products.insert(product);
		}
	});

	let sum: u32 = products.into_iter().sum();
	println!("{}", sum);
}
