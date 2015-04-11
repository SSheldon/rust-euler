extern crate euler;

use euler::is_prime;

fn main() {
	let prime = (2..).filter(|&x| is_prime(x)).nth(10000).unwrap();
	println!("{}", prime);
}
