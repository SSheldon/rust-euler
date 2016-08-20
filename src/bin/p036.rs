fn is_palindrome(s: &str) -> bool {
	s.chars().zip(s.chars().rev()).take(s.len() / 2).all(|(x, y)| x == y)
}

fn is_double_base_palindrome(n: u32) -> bool {
	is_palindrome(&n.to_string()) && is_palindrome(&format!("{:b}", n))
}

fn main() {
	let sum: u32 = (0..1000000).filter(|&x| is_double_base_palindrome(x)).sum();
	println!("{}", sum);
}
