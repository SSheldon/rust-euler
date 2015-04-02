use std::iter::AdditiveIterator;

struct Diagonals {
	current: uint,
	size: uint,
	corner: uint,
	max_size: uint,
}

impl Diagonals {
	fn new(max_size: uint) -> Diagonals {
		Diagonals{current: 1, size: 1, corner: 0, max_size: max_size}
	}
}

impl Iterator<uint> for Diagonals {
	fn next(&mut self) -> Option<uint> {
		if self.size <= self.max_size {
			self.current += self.size - 1;
			self.corner += 1;
			if self.corner >= 4 || self.size <= 1 {
				self.corner = 0;
				self.size += 2;
			}
			Some(self.current)
		} else {
			None
		}
	}
}

fn main() {
	let diag_sum = Diagonals::new(1001).sum();
	println!("{}", diag_sum);
}
