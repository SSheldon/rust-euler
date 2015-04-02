struct VecMap<'a, K, V> {
	index: 'a |&K| -> uint,
	vec: Vec<Option<V>>,
}

impl<'a, K, V> Container for VecMap<'a, K, V> {
	fn len(&self) -> uint {
		self.vec.iter().count(|x| x.is_some())
	}
}

impl<'a, K, V> Map<K, V> for VecMap<'a, K, V> {
	fn find<'r>(&'r self, key: &K) -> Option<&'r V> {
		let index = (self.index)(key);
		if index < self.vec.len() {
			self.vec.get(index).as_ref()
		} else {
			None
		}
	}
}

impl<'a, K, V> Mutable for VecMap<'a, K, V> {
	fn clear(&mut self) {
		self.vec.clear();
	}
}

impl<'a, K, V> MutableMap<K, V> for VecMap<'a, K, V> {
	fn swap(&mut self, key: K, val: V) -> Option<V> {
		let index = (self.index)(&key);
		let len = self.vec.len();
		let old = if index < len {
			self.vec.get_mut(index).take()
		} else {
			None
		};
		// It'd be nice to use grow_set here, but it requires V: Clone
		if index >= len {
			let new_len = index + 1;
			self.vec.reserve(new_len);
			for _ in range(len, new_len) {
				self.vec.push(None);
			}
		}
		*self.vec.get_mut(index) = Some(val);
		old
	}

	fn pop(&mut self, key: &K) -> Option<V> {
		let index = (self.index)(key);
		if index < self.vec.len() {
			self.vec.get_mut(index).take()
		} else {
			None
		}
	}

	fn find_mut<'r>(&'r mut self, key: &K) -> Option<&'r mut V> {
		let index = (self.index)(key);
		if index < self.vec.len() {
			self.vec.get_mut(index).as_mut()
		} else {
			None
		}
	}
}

struct Memoization<'a, A, B, T> {
	calc: 'a |A, |A| -> B| -> B,
	cache: T,
}

impl<'a, A: Clone, B: Clone, T: MutableMap<A, B>> Memoization<'a, A, B, T> {
	fn get(&mut self, arg: A) -> B {
		if self.cache.contains_key(&arg) {
			let cached = self.cache.find(&arg).unwrap();
			cached.clone()
		} else {
			let result = (self.calc)(arg.clone(), |x| self.get(x));
			self.cache.insert(arg, result.clone());
			result
		}
	}
}

fn memoize<'r, A, B>(index: 'r |&A| -> uint, calc: 'r |A, |A| -> B| -> B) -> Memoization<'r, A, B, VecMap<'r, A, B>> {
	Memoization{
		calc: calc,
		cache: VecMap{
			index: index,
			vec: Vec::new(),
		},
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
