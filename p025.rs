extern crate num;

use std::num::{Num, One, pow, Zero};
use num::bigint::{BigUint, ToBigUint};

struct Fibonacci<T> {
	current : T,
	previous : T,
}

impl<T: Num> Fibonacci<T> {
	fn new() -> Fibonacci<T> {
		Fibonacci{current: Zero::zero(), previous: One::one()}
	}
}

impl<T: Num + Clone> Iterator<T> for Fibonacci<T> {
	fn next(&mut self) -> Option<T> {
		let curr = self.current.clone();
		let next = curr + self.previous;
		self.previous = curr;
		self.current = next.clone();
		Some(next)
	}
}

fn main() {
	let mut fib = Fibonacci::<BigUint>::new();
	let threshhold = pow(10u.to_biguint().unwrap(), 999);
	let first = fib.position(|x| x >= threshhold).unwrap();
	println!("{}", first + 1);
}
