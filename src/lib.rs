use std::fmt::{Display, Formatter};

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct StringError(String);

impl Display for StringError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::error::Error for StringError {}

impl<T> From<T> for StringError
where
	T: ToOwned<Owned = String>,
{
	fn from(value: T) -> Self {
		StringError(value.to_owned())
	}
}
