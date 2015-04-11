extern crate euler;

use euler::Digits;

struct Combination<A, T> {
	outer: T,
	inner: T,
	outer_item: Option<A>,
}

impl<A, T: Iterator<Item=A> + Clone> Combination<A, T> {
	fn new(mut itr: T) -> Combination<A, T> {
		Combination{
			inner: itr.clone(),
			outer_item: itr.next(),
			outer: itr,
		}
	}
}

impl<A: Clone, T: Iterator<Item=A> + Clone> Iterator for Combination<A, T> {
	type Item = (A, A);

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
			let outer_item = self.outer_item.clone();
			match (outer_item, inner_item) {
				(Some(item1), Some(item2)) => Some((item1, item2)),
				_ => None,
			}
		}
	}
}

fn is_palindrome(n: u32) -> bool {
	let digits: Vec<_> = Digits::new(n).collect();
	digits.iter().zip(digits.iter().rev()).all(|(&x, &y)| x == y)
}

fn main() {
	let combs = Combination::new((100..1000));
	let max_palindrome = combs.map(|(x, y)| x * y)
	                          .filter(|&x| is_palindrome(x))
	                          .max().unwrap();
	println!("{}", max_palindrome);
}
