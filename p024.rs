use std::iter::{
	MultiplicativeIterator,
	range_inclusive,
};

fn lex_permutation(num_digits: uint, mut index: uint) -> ~str {
	let mut digits: Vec<uint> = range(0, num_digits).collect();
	let mut d = range_inclusive(1, num_digits).product();
	let mut result = ~"";
	for i in range_inclusive(1, num_digits).rev() {
		d /= i;
		result.push_str(digits.remove(index / d).unwrap().to_str());
		index %= d;
	}
	result
}

fn main() {
	println!("{}", lex_permutation(10, 1000000 - 1));
}
