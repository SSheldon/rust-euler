extern crate num;

use std::io::File;
use std::iter::AdditiveIterator;
use num::bigint::BigUint;

fn main() {
	let p = Path::new("p013-numbers.txt");
	let mut file = File::open(&p).unwrap();
	let contents = file.read_to_str().unwrap();

	let mut nums = contents.split_terminator('\n')
	                       .map(|x| from_str::<BigUint>(x).unwrap());
	let sum = nums.sum();
	println!("{}", sum.to_str().slice(0, 10));
}
