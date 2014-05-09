use std::cmp::max;
use std::io::File;

fn collapse_rows<T: Iterator<~[uint]>>(mut rows: T) -> uint {
	let mut collapsed = match rows.next() {
		Some(row) => row,
		None => { return 0; }
	};
	for row in rows {
		let prev = collapsed;
		// Find the max of each pair in the row
		let maxs = prev.iter().zip(prev.iter().skip(1)).map(|(&x, &y)| max(x, y));
		// Add the new row to the max values from the previous row
		collapsed = maxs.zip(row.iter()).map(|(x, &y)| x + y).collect();
	}
	// After the last row we've collapsed to a single element
	collapsed[0]
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
