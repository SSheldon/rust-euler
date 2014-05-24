extern crate num;

use std::iter::{
	AdditiveIterator,
	MultiplicativeIterator,
};
use num::bigint::ToBigUint;
use euler::Digits;

mod euler;

fn main() {
	let fac = range(1, 101).map(|x| x.to_biguint().unwrap()).product();
	let digit_sum = Digits::new(fac).sum();
	println!("{}", digit_sum);
}
