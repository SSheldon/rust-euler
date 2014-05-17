use std::iter::{
	MultiplicativeIterator,
	range_inclusive,
};

fn permutation(num_digits: uint, mut index: uint) {
	let mut digits: Vec<uint> = range(0, num_digits).collect();
	let mut d = range_inclusive(1, num_digits).product();
	for i in range_inclusive(1, num_digits).rev() {
		d /= i;
		println!("{}", digits.remove(index / d).unwrap());
		index %= d;
	}
}

fn main() {
	permutation(10, 1000000 - 1);
}
