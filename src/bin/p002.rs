extern crate euler;

use euler::Fibonacci;

fn main() {
	let fib = Fibonacci::<u32>::new();
	let sum: u32 = fib.take_while(|&x| x <= 4000000).filter(|&x| x % 2 == 0).sum();
	println!("{}", sum);
}
