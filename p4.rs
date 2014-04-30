struct CombinationRange {
	start: uint,
	stop: uint,
	first: uint,
	second: uint,
}

impl Iterator<(uint, uint)> for CombinationRange {
	fn next(&mut self) -> Option<(uint, uint)> {
		let (first, second) = if self.second < self.stop {
			(self.first, self.second)
		} else {
			(self.first + 1, self.first + 1)
		};

		if first < self.stop && second < self.stop {
			self.first = first;
			self.second = second + 1;
			Some((first, second))
		} else {
			None
		}
	}
}

fn combination_range(start: uint, stop: uint) -> CombinationRange {
	CombinationRange { start: start, stop: stop, first: start, second: start}
}

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
	let combs = combination_range(100, 1000);
	let max_palindrome = combs.map(|(x, y)| x * y)
	                          .filter(|&x| is_palindrome(x))
	                          .max().unwrap();
	println!("{}", max_palindrome);
}
