use std::iter::AdditiveIterator;

fn is_palindrome(s: &str) -> bool {
	s.chars().zip(s.chars_rev()).take(s.len() / 2).all(|(x, y)| x == y)
}

fn is_double_base_palindrome(n: uint) -> bool {
	is_palindrome(n.to_str()) && is_palindrome(format!("{:t}", n))
}

fn main() {
	let sum = range(0u, 1000000u).filter(|&x| is_double_base_palindrome(x)).sum();
	println!("{}", sum);
}
