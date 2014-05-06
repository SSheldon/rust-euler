static SIZE: uint = 21;

fn num_routes(width: uint, height: uint, cache: &mut Vec<uint>) -> uint {
	let index = SIZE * height + width;
	let cached = if index < cache.len() {
		*cache.get(index)
	} else {
		0
	};
	if cached > 0 {
		cached
	} else {
		let result = if width == 0 || height == 0 {
			1
		} else {
			let right = num_routes(width - 1, height, cache);
			let down  = num_routes(width, height - 1, cache);
			right + down
		};
		cache.grow_set(index, &0, result);
		result
	}
}

fn main() {
	let mut routes: Vec<uint> = Vec::new();
	println!("{}", num_routes(20, 20, &mut routes));
}
