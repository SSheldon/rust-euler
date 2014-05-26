struct Decimals {
	remainder: uint,
	divisor: uint,
}

impl Iterator<uint> for Decimals {
	fn next(&mut self) -> Option<uint> {
		if self.remainder > 0 {
			let digit = self.remainder / self.divisor;
			self.remainder = (self.remainder % self.divisor) * 10;
			Some(digit)
		} else {
			None
		}
	}
}

fn unit_frac_decimals(divisor: uint) -> Decimals {
	Decimals{remainder: 10, divisor: divisor}
}

fn main() {
}
