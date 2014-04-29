use std::iter::range_inclusive;
use std::num::sqrt;

fn least_divisor(n : uint) -> uint {
	match range_inclusive(2, sqrt(n as f64) as uint).find(|&x| n % x == 0) {
		Some(x) => x,
		None => n,
	}
}

fn prime_factorization(n : uint) -> ~[uint] {
	let mut remainder = n;
	let mut factors = ~[];
	loop {
		let divisor = least_divisor(remainder);
		if divisor == remainder {
			factors.push(remainder);
			break;
		} else {
			remainder /= divisor;
			factors.push(divisor);
		}
	}
	factors
}

fn main() {
	println!("{}", prime_factorization(600851475143).last().unwrap());
}
