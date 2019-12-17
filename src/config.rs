//! Parses command line arguments into a config.

use crate::error::{Error, Result};

#[derive(Clone, Copy)]
pub enum Indent {
	Tabs,
	Spaces,
}

pub type Convert = Option<(Indent, usize)>;

pub struct Config {
	/// A list of file names to fix whitespace for.
	pub fnames: Vec<String>,
	/// Whether to convert indentation.
	pub convert: Convert,
}

pub fn get() -> Result<Config> {
	get_impl(app().get_matches())
}

fn get_impl(matches: clap::ArgMatches<'static>) -> Result<Config> {
	let convert = match (matches.value_of("spaces"), matches.value_of("tabs")) {
		(None, None) => None,
		(Some(s), None) => Some((Indent::Spaces, parse_usize(s)?)),
		(None, Some(s)) => Some((Indent::Tabs, parse_usize(s)?)),
		(Some(_), Some(_)) => unreachable!(),
	};
	let fnames = matches
		.values_of("file")
		.unwrap()
		.map(ToString::to_string)
		.collect();
	Ok(Config { fnames, convert })
}

fn parse_usize(x: &str) -> Result<usize> {
	x.parse::<usize>()
		.map_err(|e| Error::ParseUsize(x.to_string(), e))
}

fn app() -> clap::App<'static, 'static> {
	// TODO use crate_authors
	clap::App::new(clap::crate_name!())
		.version(clap::crate_version!())
		.author("Ariel Davis <ariel.z.davis@icloud.com>")
		.about("Fixes whitespace")
		.arg(
			clap::Arg::with_name("spaces")
				.help("Converts a tab at line start into N spaces")
				.short("s")
				.long("spaces")
				.takes_value(true)
				.conflicts_with("tabs"),
		)
		.arg(
			clap::Arg::with_name("tabs")
				.help("Converts N spaces at line start into a tab")
				.short("t")
				.long("tabs")
				.takes_value(true),
		)
		.arg(
			clap::Arg::with_name("file")
				.help("Source file")
				.required(true)
				.multiple(true),
		)
}
