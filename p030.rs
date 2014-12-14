extern crate euler;

use std::iter::AdditiveIterator;
use euler::Digits;

static DIGIT_POWS: [uint, ..10] = [
	0,
	1,
	2*2*2*2*2,
	3*3*3*3*3,
	4*4*4*4*4,
	5*5*5*5*5,
	6*6*6*6*6,
	7*7*7*7*7,
	8*8*8*8*8,
	9*9*9*9*9,
];

fn digit_power_sum(n: uint) -> uint {
	Digits::new(n).map(|x| DIGIT_POWS[x]).sum()
}

fn main() {
	// Upper bound chosen empirically
	let sum = range(2u, 200000u).filter(|&x| x == digit_power_sum(x)).sum();
	println!("{}", sum);
}
