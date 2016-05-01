fn collatz_len(n: u64) -> u32 {
	match n {
		1 => 1,
		n if n % 2 == 0 => 1 + collatz_len(n / 2),
		n => 1 + collatz_len(3 * n + 1),
	}
}

fn main() {
	let longest = (1..1000000).max_by_key(|&x| collatz_len(x)).unwrap();
	println!("{}", longest);
}
