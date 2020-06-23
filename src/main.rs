//! A fix for whitespace.

mod args;
mod fix_ws;

fn run() -> bool {
  let args = match args::get() {
    Ok(Some(x)) => x,
    Ok(None) => return true,
    Err(e) => {
      eprintln!("{}", e);
      return false;
    }
  };
  for f in args.files {
    let bs = match std::fs::read(&f) {
      Ok(x) => x,
      Err(e) => {
        eprintln!("{}: {}", f, e);
        return false;
      }
    };
    let bs = fix_ws::get(&bs, args.convert);
    match std::fs::write(&f, &bs) {
      Ok(()) => {}
      Err(e) => {
        eprintln!("{}: {}", f, e);
        return false;
      }
    }
  }
  true
}

fn main() {
  if !run() {
    std::process::exit(1);
  }
}
