#![feature(iter_arith)]

use std::fs::File;
use std::io::Read;

fn name_score(name: &str) -> u32 {
	name.chars().map(|c| (c as u32) + 1 - ('A' as u32)).sum()
}

fn main() {
	let mut file = File::open("p022-names.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	let mut names: Vec<&str> = contents.split_terminator('\n').collect();
	names.sort();
	let total: u32 = names.into_iter()
		.enumerate()
		.map(|(i, x)| (i as u32 + 1) * name_score(x))
		.sum();
	println!("{}", total);
}
