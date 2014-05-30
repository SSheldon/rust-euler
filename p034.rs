use std::iter::AdditiveIterator;
use euler::Digits;

mod euler;

static DIGIT_FACS: [uint, ..10] = [
	1,
	1,
	1*2,
	1*2*3,
	1*2*3*4,
	1*2*3*4*5,
	1*2*3*4*5*6,
	1*2*3*4*5*6*7,
	1*2*3*4*5*6*7*8,
	1*2*3*4*5*6*7*8*9,
];

fn digit_fac_sum(n: uint) -> uint {
	Digits::new(n).map(|x| DIGIT_FACS[x]).sum()
}

fn main() {
	// Upper bound chosen empirically
	let sum = range(10u, 50000u).filter(|&x| x == digit_fac_sum(x)).sum();
	println!("{}", sum);
}
