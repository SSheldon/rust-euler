extern crate euler;

use euler::Digits;

static DIGIT_FACS: [u32; 10] = [
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

fn digit_fac_sum(n: u32) -> u32 {
	Digits::new(n).map(|x| DIGIT_FACS[x as usize]).sum()
}

fn main() {
	// Upper bound chosen empirically
	let sum: u32 = (10..50000).filter(|&x| x == digit_fac_sum(x)).sum();
	println!("{}", sum);
}
