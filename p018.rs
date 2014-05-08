use std::io::File;

fn collapse_rows<T: Iterator<~[uint]>>(mut rows: T) -> uint {
	0
}

fn nums_from_line(line: &str) -> ~[uint] {
	line.split(' ').map(|x| from_str(x).unwrap()).collect()
}

fn main() {
	let p = Path::new("p018-triangle.txt");
	let mut file = File::open(&p).unwrap();
	let contents = file.read_to_str().unwrap();

	let rows = contents.split('\n').map(nums_from_line);
	let max_path_sum = collapse_rows(rows);
	println!("{}", max_path_sum);
}
