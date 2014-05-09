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
		if index < len {
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
