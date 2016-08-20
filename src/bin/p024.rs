extern crate num;

use num::range_inclusive;

fn lex_permutation(num_digits: u32, mut index: u32) -> String {
	let mut digits: Vec<u32> = (0..num_digits).collect();
	let mut d: u32 = range_inclusive(1, num_digits).product();
	let mut result = String::new();
	for i in range_inclusive(1, num_digits).rev() {
		d /= i;
		result.push_str(&digits.remove((index / d) as usize).to_string());
		index %= d;
	}
	result
}

fn main() {
	println!("{}", lex_permutation(10, 1000000 - 1));
}
