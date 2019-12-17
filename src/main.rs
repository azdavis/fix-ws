//! Removes trailing whitespace and adds a newline at EOF.

mod config;
mod error;
mod file;
mod fix_ws;

use error::Result;

fn run() -> Result<()> {
	for fname in config::get()?.fnames.iter() {
		let bs = file::read(fname)?;
		let bs = fix_ws::get(&bs);
		file::write(fname, &bs)?;
	}
	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
		std::process::exit(1);
	}
}
