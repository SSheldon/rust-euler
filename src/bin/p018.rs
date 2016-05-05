use std::cmp::max;
use std::fs::File;
use std::io::Read;

fn collapse_rows<T: Iterator<Item=Vec<u32>>>(mut rows: T) -> u32 {
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

fn nums_from_line(line: &str) -> Vec<u32> {
	line.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn main() {
	let mut file = File::open("p018-triangle.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	let rows = contents.split('\n').map(nums_from_line);
	let max_path_sum = collapse_rows(rows);
	println!("{}", max_path_sum);
}
