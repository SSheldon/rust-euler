fn collatz_len(n: uint) -> uint {
	match n {
		1 => 1,
		n if n % 2 == 0 => 1 + collatz_len(n / 2),
		n => 1 + collatz_len(3 * n + 1),
	}
}

fn main() {
	let longest = range(1u, 1000000u).max_by(|&x| collatz_len(x)).unwrap();
	println!("{}", longest);
}
