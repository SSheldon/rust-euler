use std::iter::AdditiveIterator;
use euler::Fibonacci;

mod euler;

fn main() {
	let fib = Fibonacci::<uint>::new();
	let sum = fib.take_while(|&x| x <= 4000000).filter(|&x| x % 2 == 0).sum();
	println!("{}", sum);
}
