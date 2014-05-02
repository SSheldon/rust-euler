use std::iter::AdditiveIterator;

fn main() {
	let sum = range(1, 101).sum();
	let square_of_sum = sum * sum;
	let sum_of_squares = range(1, 101).map(|x| x * x).sum();
	let diff = square_of_sum - sum_of_squares;
	println!("{}", diff);
}
