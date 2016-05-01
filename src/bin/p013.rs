extern crate num;

use std::io::Read;
use std::fs::File;
use num::Zero;
use num::bigint::BigUint;

fn main() {
	let mut file = File::open("p013-numbers.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	let sum = contents.split_terminator('\n')
		.map(|x| x.parse::<BigUint>().unwrap())
		.fold(BigUint::zero(), |a, b| a + b);
	println!("{}", &sum.to_string()[0..10]);
}
