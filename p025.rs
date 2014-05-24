extern crate num;

use std::num::pow;
use num::bigint::{BigUint, ToBigUint};
use euler::Fibonacci;

mod euler;

fn main() {
	let mut fib = Fibonacci::<BigUint>::new();
	let threshhold = pow(10u.to_biguint().unwrap(), 999);
	let first = fib.position(|x| x >= threshhold).unwrap();
	println!("{}", first + 1);
}
