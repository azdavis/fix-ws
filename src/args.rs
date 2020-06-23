//! Command-line arguments.

use gumdrop::Options;

pub fn get() -> Result<Args, bool> {
  let raw = RawArgs::parse_args_default_or_exit();
  if raw.version {
    println!("{}", env!("CARGO_PKG_VERSION"));
    return Err(true);
  }
  let convert = match (raw.spaces, raw.tabs) {
    (None, None) => None,
    (Some(x), None) => Some((Indent::Spaces, x)),
    (None, Some(x)) => Some((Indent::Tabs, x)),
    (Some(..), Some(..)) => {
      eprintln!("cannot pass both --spaces and --tabs");
      return Err(false);
    }
  };
  Ok(Args {
    convert,
    files: raw.files,
  })
}

#[derive(Options)]
struct RawArgs {
  #[options(help = "Show this help")]
  help: bool,
  #[options(help = "Show the version")]
  version: bool,
  #[options(help = "Use spaces to indent")]
  spaces: Option<u8>,
  #[options(help = "Use tabs to indent")]
  tabs: Option<u8>,
  #[options(free, help = "Files")]
  files: Vec<String>,
}

pub struct Args {
  pub convert: Convert,
  pub files: Vec<String>,
}

#[derive(Clone, Copy)]
pub enum Indent {
  Spaces,
  Tabs,
}

pub type Convert = Option<(Indent, u8)>;
