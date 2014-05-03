use std::iter::{
	count,
	range_inclusive,
};
use std::num::sqrt;

fn is_prime(n: uint) -> bool {
	range_inclusive(2, sqrt(n as f64) as uint).all(|x| n % x != 0)
}

fn main() {
	let prime = count(2u, 1).filter(|&x| is_prime(x)).nth(10000).unwrap();
	println!("{}", prime);
}
