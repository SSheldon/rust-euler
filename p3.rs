use std::iter::range_inclusive;

fn least_divisor(n : uint) -> uint {
	match range_inclusive(2, n / 2).find(|&x| n % x == 0) {
		Some(x) => x,
		None => n,
	}
}

fn main() {
	let mut remainder = 600851475143;
	loop {
		let divisor = least_divisor(remainder);
		if divisor == remainder {
			println!("{}", remainder);
			break;
		} else {
			remainder /= divisor;
		}
	}
}
