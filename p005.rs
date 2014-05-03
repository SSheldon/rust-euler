use std::iter::count;

fn main() {
	let min = count(1, 1).find(|&x| range(1, 21).all(|y| x % y == 0)).unwrap();
	println!("{}", min);
}
