extern crate num;

use std::iter::{MultiplicativeIterator, Repeat};
use num::rational::Ratio;

fn digits_cancel(numer: uint, denom: uint) -> bool {
	let (n1, n0) = (numer / 10, numer % 10);
	let (d1, d0) = (denom / 10, denom % 10);
	(n1 == d1 && n1 != 0 && numer * d0 == n0 * denom) ||
		(n1 == d0 && n1 != 0 && numer * d1 == n0 * denom) ||
		(n0 == d1 && n0 != 0 && numer * d0 == n1 * denom) ||
		(n0 == d0 && n0 != 0 && numer * d1 == n1 * denom)
}

fn main() {
	let product = range(10u, 100u)
		.flat_map(|n| Repeat::new(n).zip(range(n + 1, 100u)))
		.filter(|&(n, d)| digits_cancel(n, d))
		.map(|(n, d)| Ratio::new(n, d))
		.product().reduced();
	println!("{}", product.denom());
}
