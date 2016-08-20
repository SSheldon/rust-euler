extern crate num;

use std::iter::repeat;
use num::One;
use num::rational::Ratio;

fn digits_cancel(numer: u32, denom: u32) -> bool {
	let (n1, n0) = (numer / 10, numer % 10);
	let (d1, d0) = (denom / 10, denom % 10);
	(n1 == d1 && n1 != 0 && numer * d0 == n0 * denom) ||
		(n1 == d0 && n1 != 0 && numer * d1 == n0 * denom) ||
		(n0 == d1 && n0 != 0 && numer * d0 == n1 * denom) ||
		(n0 == d0 && n0 != 0 && numer * d1 == n1 * denom)
}

fn main() {
	let product = (10..100)
		.flat_map(|n| repeat(n).zip(n+1..100))
		.filter(|&(n, d)| digits_cancel(n, d))
		.map(|(n, d)| Ratio::new(n, d))
		.fold(Ratio::one(), |a, b| a * b)
		.reduced();
	println!("{}", product.denom());
}
