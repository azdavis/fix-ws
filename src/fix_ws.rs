//! The core logic.

use crate::args::{Convert, Indent};

pub fn get(bs: &[u8], convert: Convert) -> Vec<u8> {
  let mut ret = Vec::with_capacity(bs.len());
  let mut cur_line_ws = Vec::new();
  let mut cur_line_just_ws = true;
  let mut new_lines = 0;
  for &b in bs {
    if b == b'\r' {
      continue;
    }
    if b == b'\n' {
      cur_line_ws.clear();
      new_lines += 1;
      cur_line_just_ws = true;
      continue;
    }
    if b.is_ascii_whitespace() {
      cur_line_ws.push(b);
      continue;
    }
    ret.resize(ret.len() + new_lines, b'\n');
    new_lines = 0;
    cur_line_ws = if cur_line_just_ws {
      match convert {
        None => cur_line_ws,
        Some((Indent::Spaces, n)) => convert_to_spaces(&cur_line_ws, n),
        Some((Indent::Tabs, n)) => convert_to_tabs(&cur_line_ws, n),
      }
    } else {
      cur_line_ws
    };
    cur_line_just_ws = false;
    ret.append(&mut cur_line_ws);
    ret.push(b);
  }
  if let Some(&b) = ret.last() {
    if b != b'\n' {
      ret.push(b'\n');
    }
  }
  ret.shrink_to_fit();
  ret
}

fn convert_to_spaces(bs: &[u8], n: u8) -> Vec<u8> {
  let mut ret = Vec::with_capacity(bs.len());
  for &b in bs {
    if b == b'\t' {
      ret.resize(ret.len() + (n as usize), b' ');
      continue;
    }
    ret.push(b);
  }
  ret
}

fn convert_to_tabs(bs: &[u8], n: u8) -> Vec<u8> {
  let mut ret = Vec::with_capacity(bs.len());
  let mut consec_spaces = 0;
  for &b in bs {
    if b == b' ' {
      consec_spaces += 1;
      if consec_spaces == n {
        consec_spaces = 0;
        ret.push(b'\t');
      }
      continue;
    }
    ret.resize(ret.len() + (consec_spaces as usize), b' ');
    consec_spaces = 0;
    ret.push(b);
  }
  ret.resize(ret.len() + (consec_spaces as usize), b' ');
  ret.shrink_to_fit();
  ret
}
