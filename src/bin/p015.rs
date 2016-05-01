struct VecMap<K, V> {
	index: Box<Fn(&K) -> usize>,
	vec: Vec<Option<V>>,
}

impl<K, V> VecMap<K, V> {
	fn new<F>(index: F) -> VecMap<K, V>
			where F: 'static + Fn(&K) -> usize {
		VecMap {
			index: Box::new(index),
			vec: Vec::new(),
		}
	}

	fn find<'r>(&'r self, key: &K) -> Option<&'r V> {
		let index = (self.index)(key);
		if index < self.vec.len() {
			self.vec[index].as_ref()
		} else {
			None
		}
	}

	fn insert(&mut self, key: K, val: V) {
		let index = (self.index)(&key);
		let len = self.vec.len();
		// It'd be nice to use grow_set here, but it requires V: Clone
		if index >= len {
			let new_len = index + 1;
			self.vec.reserve(new_len);
			for _ in len..new_len {
				self.vec.push(None);
			}
		}
		self.vec[index] = Some(val);
	}
}

struct Memoization<A, B> {
	calc: fn(A, &mut FnMut(A) -> B) -> B,
	cache: VecMap<A, B>,
}

impl<A: Clone, B: Clone> Memoization<A, B> {
	fn get(&mut self, arg: A) -> B {
		if let Some(cached) = self.cache.find(&arg) {
			return cached.clone();
		}

		let result = (self.calc)(arg.clone(), &mut |x| self.get(x));
		self.cache.insert(arg, result.clone());
		result
	}
}

fn memoize<A, B, F>(index: F, calc: fn(A, &mut FnMut(A) -> B) -> B) -> Memoization<A, B>
		where F: 'static + Fn(&A) -> usize {
	Memoization{
		calc: calc,
		cache: VecMap::new(index),
	}
}

const SIZE: u32 = 21;

fn num_routes((w, h): (u32, u32), f: &mut FnMut((u32, u32)) -> u64) -> u64 {
	if w == 0 || h == 0 {
		1
	} else {
		let right = f((w - 1, h));
		let down  = f((w, h - 1));
		right + down
	}
}

fn main() {
	let mut mem = memoize(
		|&(w, h)| (SIZE * h + w) as usize,
		num_routes,
	);
	println!("{}", mem.get((20, 20)));
}
