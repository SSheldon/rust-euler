fn main() {
	let mut sum = 0;
	let mut i = 3;
	while i < 1000 {
		sum += i;
		i += 3;
	}
	i = 5;
	while i < 1000 {
		if i % 3 != 0 {
			sum += i;
		}
		i += 5;
	}
	println!("{}", sum);
}
