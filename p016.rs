extern crate num;

use std::iter::{
	AdditiveIterator,
	MultiplicativeIterator,
	Repeat,
};
use num::bigint::ToBigUint;
use euler::Digits;

mod euler;

fn main() {
	let pow = Repeat::new(2u.to_biguint().unwrap()).take(1000).product();
	let digit_sum = Digits::new(pow).sum();
	println!("{}", digit_sum);
}
