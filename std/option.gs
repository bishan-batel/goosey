
pub enum Option<T> {
	Some(T),
	None
}

impl Option<T> {
	pub fun is_some(ref[self]): bool =>	match self {
			Some(_) => true 
		  None => false
	}

	pub fun is_none(ref[self]): bool => match self {
			Some(_) => false
		  None => true
	}

	pub fun unwrap(self): T =>	match self {
			Some(v) => v
		  None => crash!("Unwrapped none option")
	}

	pub fun or_default(self, default: T): T => match self {
			Some(v) => v
		  None => default
	}
}
