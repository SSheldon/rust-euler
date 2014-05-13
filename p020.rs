extern crate num;

use std::iter::{
	AdditiveIterator,
	MultiplicativeIterator,
};
use std::num::One;
use num::Integer;
use num::bigint::ToBigUint;

struct Digits<T> {
	remainder: T,
}

impl<T: Integer> Iterator<T> for Digits<T> {
	fn next(&mut self) -> Option<T> {
		if !self.remainder.is_zero() {
			let one: |int| -> T = |_| { One::one() };
			let ten: T = range(0, 10).map(one).sum(); // Lol
			let digit = self.remainder % ten;
			self.remainder = self.remainder / ten;
			Some(digit)
		} else {
			None
		}
	}
}

fn main() {
	let fac = range(1, 101).map(|x| x.to_biguint().unwrap()).product();
	let digit_sum = Digits{remainder: fac}.sum();
	println!("{}", digit_sum);
}
