struct Memoization<'a, A, B> {
	cache: Vec<Option<B>>,
	index: 'a |&A| -> uint,
	calc: 'a |A, |A| -> B| -> B,
}

impl<'a, A, B: Clone> Memoization<'a, A, B> {
	fn get(&mut self, arg: A) -> B {
		let index = (self.index)(&arg);
		if index >= self.cache.len() || self.cache.get(index).is_none() {
			let result = (self.calc)(arg, |x| self.get(x));
			self.cache.grow_set(index, &None, Some(result));
		}
		let cached = self.cache.get(index);
		cached.get_ref().clone()
	}
}

fn memoize<'r, A, B>(index: 'r |&A| -> uint, calc: 'r |A, |A| -> B| -> B) -> Memoization<'r, A, B> {
	Memoization{
		cache: Vec::new(),
		index: index,
		calc: calc,
	}
}

static SIZE: uint = 21;

fn num_routes((w, h): (uint, uint), f: |(uint, uint)| -> uint) -> uint {
	if w == 0 || h == 0 {
		1
	} else {
		let right = f((w - 1, h));
		let down  = f((w, h - 1));
		right + down
	}
}

fn main() {
	let mut mem = memoize::<(uint, uint), uint>(
		|&(w, h)| SIZE * h + w,
		num_routes,
	);
	println!("{}", mem.get((20, 20)));
}
