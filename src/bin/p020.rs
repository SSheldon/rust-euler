#![feature(iter_arith)]

extern crate euler;
extern crate num;

use num::{One, ToPrimitive};
use num::bigint::BigUint;
use euler::Digits;

fn main() {
	let fac = (1u32..101).map(|x| BigUint::from(x))
		.fold(BigUint::one(), |a, b| a * b);
	let digit_sum: u32 = Digits::new(fac).map(|d| d.to_u32().unwrap()).sum();
	println!("{}", digit_sum);
}
