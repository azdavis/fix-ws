//! A fix for whitespace.

mod config;
mod error;
mod file;
mod fix_ws;

use error::Result;

fn run() -> Result<()> {
	let c = config::get()?;
	for fname in c.fnames.iter() {
		let bs = file::read(fname)?;
		let bs = fix_ws::get(&bs, c.convert);
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
