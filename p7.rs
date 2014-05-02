use std::iter::count;

fn is_prime(n: uint) -> bool {
	range(2, n).all(|x| n % x != 0)
}

fn main() {
	let prime = count(2u, 1).filter(|&x| is_prime(x)).nth(10000).unwrap();
	println!("{}", prime);
}
