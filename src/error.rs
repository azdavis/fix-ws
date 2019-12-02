//! Errors.

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
	Io(String, io::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(fname, err) => write!(f, "{}: {}", fname, err),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Io(_, err) => Some(err),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
