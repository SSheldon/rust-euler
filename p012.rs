use std::iter::range_inclusive;

fn num_factors(n: uint) -> uint {
	// Only factor after n/2 is n, so just stop halfway and add 1
	1 + range_inclusive(1, n / 2).count(|x| n % x == 0)
}

fn main() {
}
