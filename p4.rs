struct Combination<A, T> {
	outer: T,
	inner: T,
	outer_item: Option<A>,
}

impl<A, T: Iterator<A> + Clone> Combination<A, T> {
	fn new(mut itr: T) -> Combination<A, T> {
		Combination{
			inner: itr.clone(),
			outer_item: itr.next(),
			outer: itr,
		}
	}
}

impl<A: Clone, T: Iterator<A> + Clone> Iterator<(A, A)> for Combination<A, T> {
	fn next(&mut self) -> Option<(A, A)> {
		if self.outer_item.is_none() {
			None
		} else {
			let inner_item = match self.inner.next() {
				Some(item) => Some(item),
				None => {
					self.inner = self.outer.clone();
					self.outer_item = self.outer.next();
					self.inner.next()
				}
			};
			match (self.outer_item.clone(), inner_item) {
				(Some(item1), Some(item2)) => Some((item1, item2)),
				_ => None,
			}
		}
	}
}

trait CombinableIterator<A> {
	fn combination(self) -> Combination<A, Self>;
}

impl<A, T: Iterator<A> + Clone> CombinableIterator<A> for T {
	fn combination(self) -> Combination<A, T> {
		Combination::new(self)
	}
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
	let combs = range(100u, 1000u).combination();
	let max_palindrome = combs.map(|(x, y)| x * y)
	                          .filter(|&x| is_palindrome(x))
	                          .max().unwrap();
	println!("{}", max_palindrome);
}
