use std::io::File;
use std::iter::AdditiveIterator;

fn name_score(name: &str) -> uint {
	name.chars().map(|c| (c as uint) + 1 - ('A' as uint)).sum()
}

fn main() {
	let p = Path::new("p022-names.txt");
	let mut file = File::open(&p).unwrap();
	let contents = file.read_to_str().unwrap();

	let mut names: Vec<&str> = contents.split_terminator('\n').collect();
	names.sort();
	let total = names.move_iter()
	                 .enumerate()
	                 .map(|(i, x)| (i + 1) * name_score(x))
	                 .sum();
	println!("{}", total);
}
