extern crate num;

use num::Integer;

fn digits_cancel(numer: uint, denom: uint) -> bool {
	let (n1, n0) = (numer / 10, numer % 10);
	let (d1, d0) = (denom / 10, denom % 10);
	(n1 == d1 && n1 != 0 && numer * d0 == n0 * denom) ||
		(n1 == d0 && n1 != 0 && numer * d1 == n0 * denom) ||
		(n0 == d1 && n0 != 0 && numer * d0 == n1 * denom) ||
		(n0 == d0 && n0 != 0 && numer * d1 == n1 * denom)
}

fn main() {
	let mut numer = 1;
	let mut denom = 1;
	for n in range(10u, 100u) {
		for d in range(n + 1, 100u) {
			if digits_cancel(n, d) {
				numer *= n;
				denom *= d;
			}
		}
	}
	let gcd = numer.gcd(&denom);
	println!("{}", denom / gcd);
}
