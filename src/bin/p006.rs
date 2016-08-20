fn main() {
	let sum: u32 = (1..101).sum();
	let square_of_sum = sum * sum;
	let sum_of_squares: u32 = (1..101).map(|x| x * x).sum();
	let diff = square_of_sum - sum_of_squares;
	println!("{}", diff);
}
