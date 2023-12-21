import std::ops::Drop

struct RCInternal<T> {
	ref_count: usize
	value: T
}

pub struct Rc<T> {
	ptr: Box<RCInternal<T>>
}

impl Rc<T> {
	pub fun of(arg: T): Rc<T> => make Rc<T> {
		ptr: alloc RCInternal<T> {
			ref_count: 1 
			value: arg
		}
	}
}

impl Drop for Rc {
	fun drop(self) {
		self.ref_count -= 1;
	}
}

