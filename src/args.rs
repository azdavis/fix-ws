//! Command-line arguments.

pub fn get() -> Result<Option<Args>, Box<dyn std::error::Error>> {
  let mut args = pico_args::Arguments::from_env();
  if args.contains(["-h", "--help"]) {
    print!("{}", include_str!("help.txt"));
    return Ok(None);
  }
  if args.contains(["-v", "--version"]) {
    println!("{}", env!("CARGO_PKG_VERSION"));
    return Ok(None);
  }
  let spaces: Option<u8> = args.opt_value_from_str(["-s", "--spaces"])?;
  let tabs: Option<u8> = args.opt_value_from_str(["-t", "--tabs"])?;
  let convert = match (spaces, tabs) {
    (None, None) => None,
    (Some(x), None) => Some((Indent::Spaces, x)),
    (None, Some(x)) => Some((Indent::Tabs, x)),
    (Some(..), Some(..)) => return Err("cannot pass both --spaces and --tabs".into()),
  };
  Ok(Some(Args {
    convert,
    files: args.finish(),
  }))
}

pub struct Args {
  pub convert: Convert,
  pub files: Vec<std::ffi::OsString>,
}

#[derive(Clone, Copy)]
pub enum Indent {
  Spaces,
  Tabs,
}

pub type Convert = Option<(Indent, u8)>;
