 pub enum Result<T, E> {
	Ok(T),
	Err(E)
}

impl Result<T, E> {
	pub fun err(ref[self]): bool => match self {
		 Ok(_) => false
		 Err(_) => false
	}
	

	pub fun unwrap(self) -> T {
	}
}
