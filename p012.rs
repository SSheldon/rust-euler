use std::iter::range_inclusive;
use std::num::sqrt;

fn is_triangular(n: uint) -> bool {
	// If n is triangular, there must be some i such that i(i+1)/2=n
	// So by the quadratic formula, (sqrt(1+8n)-1)/2 must be an integer
	let d = 8 * n + 1;
	let sqrt_d = sqrt(d as f64) as uint;
	(sqrt_d * sqrt_d == d) && ((sqrt_d - 1) % 2 == 0)
}

fn num_factors(n: uint) -> uint {
	// Only factor after n/2 is n, so just stop halfway and add 1
	1 + range_inclusive(1, n / 2).count(|x| n % x == 0)
}

fn main() {
}
