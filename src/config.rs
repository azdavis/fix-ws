//! Parses command line arguments into a config.

pub struct Config {
	/// A list of file names to fix whitespace for.
	pub fnames: Vec<String>,
}

pub fn get() -> Result<Config> {
	get_impl(app().get_matches())
}

fn get_impl(matches: clap::ArgMatches<'static>) -> Result<Config> {
	let fnames = matches
		.values_of("file")
		.unwrap()
		.map(ToString::to_string)
		.collect();
	Ok(Config { fnames, convert })
}

fn app() -> clap::App<'static, 'static> {
	// TODO use crate_authors
	clap::App::new(clap::crate_name!())
		.version(clap::crate_version!())
		.author("Ariel Davis <ariel.z.davis@icloud.com>")
		.about("Fixes whitespace")
		.arg(
			clap::Arg::with_name("file")
				.help("Source file")
				.required(true)
				.multiple(true),
		)
}
