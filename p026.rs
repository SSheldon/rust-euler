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

fn recurring_cycle<A, T: Iterator<A>>(itr: T) -> Option<Vec<A>> {
	None
}

fn main() {
	let max_d = range(2u, 1000u).max_by(|&d| {
		let decimals = unit_frac_decimals(d);
		match recurring_cycle(decimals) {
			Some(cycle) => cycle.len(),
			None => 0,
		}
	}).unwrap();
	println!("{}", max_d);
}
