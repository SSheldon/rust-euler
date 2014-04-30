struct Digits {
	remainder: uint,
}

impl Iterator<uint> for Digits {
	fn next(&mut self) -> Option<uint> {
		if self.remainder > 0 {
			let digit = self.remainder % 10;
			self.remainder /= 10;
			Some(digit)
		} else {
			None
		}
	}
}

fn is_palindrome(n: uint) -> bool {
	let mut digit_iter = Digits { remainder: n };
	let digits: ~[uint] = digit_iter.collect();
	digits.iter().zip(digits.iter().rev()).all(|(&x, &y)| x == y)
}

fn main() {
}
