use std::iter::CloneableIterator;

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

fn is_recurring_cycle<A: Eq, T: Iterator<A>>(mut itr: T, cycle: &[A]) -> bool {
	let cycle_itr = cycle.iter().cycle();
	// Can't check infinitely, but check it cycles 3 times
	for x in cycle_itr.take(cycle.len() * 3) {
		match itr.next() {
			None => { return false; }
			Some(ref y) if x != y => { return false; }
			Some(_) => { continue; }
		}
	}
	true
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
