use std::iter::AdditiveIterator;

fn main() {
	let sum = range(0, 1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
	println!("{}", sum);
}