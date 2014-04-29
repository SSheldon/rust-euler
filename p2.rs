use std::iter::AdditiveIterator;

struct Fibonacci {
	current : uint,
	previous : uint,
}

impl Iterator<uint> for Fibonacci {
	fn next(&mut self) -> Option<uint> {
		let next = self.previous + self.current;
		self.previous = self.current;
		self.current = next;
		Some(next)
	}
}

fn main() {
	let fib = Fibonacci { previous: 0, current: 1 };
	let sum = fib.take_while(|&x| x <= 4000000).filter(|&x| x % 2 == 0).sum();
	println!("{}", sum);
}
