fn recurring_cycle_len(d: uint) -> uint {
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
	let max_d = range(2u, 1000u).max_by(|&d| recurring_cycle_len(d)).unwrap();
	println!("{}", max_d);
}
