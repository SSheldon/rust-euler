extern crate vec_map;

use std::cell::RefCell;
use vec_map::VecMap;

struct Memoization<A, B> {
	calc: Box<Fn(A, &Memoization<A, B>) -> B>,
	index: Box<Fn(&A) -> usize>,
	cache: RefCell<VecMap<B>>,
}

impl<A: Clone, B: Clone> Memoization<A, B> {
	fn new<I, C>(index: I, calc: C) -> Self
			where I: 'static + Fn(&A) -> usize,
			      C: 'static + Fn(A, &Self) -> B {
		Memoization{
			calc: Box::new(calc),
			index: Box::new(index),
			cache: RefCell::new(VecMap::new()),
		}
	}

	fn get(&self, arg: A) -> B {
		let index = (self.index)(&arg);
		if let Some(cached) = self.cache.borrow().get(index) {
			return cached.clone();
		}

		let result = (self.calc)(arg.clone(), self);
		self.cache.borrow_mut().insert(index, result.clone());
		result
	}
}

const SIZE: u32 = 21;

fn num_routes((w, h): (u32, u32), f: &Memoization<(u32, u32), u64>) -> u64 {
	if w == 0 || h == 0 {
		1
	} else {
		let right = f.get((w - 1, h));
		let down  = f.get((w, h - 1));
		right + down
	}
}

fn main() {
	let mem = Memoization::new(
		|&(w, h)| (SIZE * h + w) as usize,
		num_routes,
	);
	println!("{}", mem.get((20, 20)));
}
