use std::iter::{
	AdditiveIterator,
	range_inclusive,
};
use std::num::sqrt;

fn is_prime(n: uint) -> bool {
	range_inclusive(2, sqrt(n as f64) as uint).all(|x| n % x != 0)
}

fn main() {
	let sum = range(2u, 2000000u).filter(|&x| is_prime(x)).sum();
	println!("{}", sum);
}
