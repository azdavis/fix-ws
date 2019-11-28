//! Quick and dirty file operations.

use crate::error::{Error, Result};

pub fn read(fname: &str) -> Result<Vec<u8>> {
	std::fs::read(fname).map_err(|e| Error::Io(fname.to_string(), e))
}

pub fn write(fname: &str, bs: &[u8]) -> Result<()> {
	std::fs::write(fname, bs).map_err(|e| Error::Io(fname.to_string(), e))
}
