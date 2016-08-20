struct Diagonals {
	current: u32,
	size: u32,
	corner: u32,
	max_size: u32,
}

impl Diagonals {
	fn new(max_size: u32) -> Diagonals {
		Diagonals{current: 1, size: 1, corner: 0, max_size: max_size}
	}
}

impl Iterator for Diagonals {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
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
	let diag_sum: u32 = Diagonals::new(1001).sum();
	println!("{}", diag_sum);
}
