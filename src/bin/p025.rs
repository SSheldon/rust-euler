extern crate euler;
extern crate num;

use num::pow;
use num::bigint::BigUint;
use euler::Fibonacci;

fn main() {
	let mut fib = Fibonacci::<BigUint>::new();
	let threshhold = pow(BigUint::from(10u32), 999);
	let first = fib.position(|x| x >= threshhold).unwrap();
	println!("{}", first + 1);
}
