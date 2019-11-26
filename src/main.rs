//! Removes trailing whitespace.

mod error;
mod file;
mod rm_ws;

use error::Result;

fn run_one(fname: &str) -> Result<()> {
	let bs = file::read(fname)?;
	let bs = rm_ws::get(bs);
	file::write(fname, &bs)?;
	Ok(())
}

fn run() -> Result<()> {
	for arg in std::env::args() {
		run_one(&arg)?;
	}
	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
		std::process::exit(1);
	}
}
