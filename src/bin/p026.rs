fn recurring_cycle_len(d: u32) -> usize {
	let mut rem = 1;
	let mut prev_rems = Vec::new();
	while rem > 0 {
		// If we have a remainder we've already seen, it's a cycle
		match prev_rems.iter().position(|&x| rem == x) {
			Some(x) => { return prev_rems.len() - x; }
			None => (),
		}
		prev_rems.push(rem);
		rem = (rem * 10) % d;
	}
	0
}

fn main() {
	let max_d = (2..1000).max_by_key(|&d| recurring_cycle_len(d)).unwrap();
	println!("{}", max_d);
}
