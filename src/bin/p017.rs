#![feature(iter_arith)]

fn written_len(n: u32) -> usize {
	match n {
		0 => 0,
		1 => "one".len(),
		2 => "two".len(),
		3 => "three".len(),
		4 => "four".len(),
		5 => "five".len(),
		6 => "six".len(),
		7 => "seven".len(),
		8 => "eight".len(),
		9 => "nine".len(),
		10 => "ten".len(),
		11 => "eleven".len(),
		12 => "twelve".len(),
		13 => "thirteen".len(),
		14 => "fourteen".len(),
		15 => "fifteen".len(),
		16 => "sixteen".len(),
		17 => "seventeen".len(),
		18 => "eighteen".len(),
		19 => "nineteen".len(),
		20 => "twenty".len(),
		30 => "thirty".len(),
		40 => "forty".len(),
		50 => "fifty".len(),
		60 => "sixty".len(),
		70 => "seventy".len(),
		80 => "eighty".len(),
		90 => "ninety".len(),
		n if n < 100 =>
			written_len(n / 10 * 10) + written_len(n % 10),
		n if n < 1000 && n % 100 == 0 =>
			written_len(n / 100) + "hundred".len(),
		n if n < 1000 =>
			written_len(n / 100) + "hundredand".len() + written_len(n % 100),
		n =>
			written_len(n / 1000) + "thousand".len() + written_len(n % 1000),
	}
}

fn main() {
	let sum: usize = (1..1001).map(written_len).sum();
	println!("{}", sum);
}
