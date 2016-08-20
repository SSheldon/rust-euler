extern crate euler;

use euler::Digits;

static DIGIT_POWS: [u32; 10] = [
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

fn digit_power_sum(n: u32) -> u32 {
	Digits::new(n).map(|x| DIGIT_POWS[x as usize]).sum()
}

fn main() {
	// Upper bound chosen empirically
	let sum: u32 = (2..200000).filter(|&x| x == digit_power_sum(x)).sum();
	println!("{}", sum);
}
