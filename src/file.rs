//! Quick and dirty file operations.

use crate::error::{Error, Result};
use std::fs::File;
use std::io::Read as _;
use std::io::Write as _;

pub fn read(fname: &str) -> Result<Vec<u8>> {
	let mut f = match File::open(fname) {
		Ok(f) => f,
		Err(e) => return Err(Error::Io(fname.to_string(), e)),
	};
	let mut bs = Vec::new();
	match f.read_to_end(&mut bs) {
		Ok(f) => f,
		Err(e) => return Err(Error::Io(fname.to_string(), e)),
	};
	Ok(bs)
}

pub fn write(fname: &str, bs: &[u8]) -> Result<()> {
	let mut f = match File::create(fname) {
		Ok(f) => f,
		Err(e) => return Err(Error::Io(fname.to_string(), e)),
	};
	match f.write_all(bs) {
		Ok(()) => (),
		Err(e) => return Err(Error::Io(fname.to_string(), e)),
	}
	Ok(())
}
