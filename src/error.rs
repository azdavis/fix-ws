//! Errors.

use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
	Io(String, io::Error),
	ParseUsize(String, num::ParseIntError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(fname, err) => write!(f, "{}: {}", fname, err),
			Self::ParseUsize(x, e) => write!(f, "{} is not a usize: {}", x, e),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Io(_, err) => Some(err),
			Self::ParseUsize(_, err) => Some(err),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
