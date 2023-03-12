use crate::args::Indent;
use crate::fix_ws::get;

#[test]
fn crlf() {
  let inp = include_bytes!("test_inputs/crlf/inp.txt");
  let out = include_bytes!("test_inputs/crlf/out.txt").to_vec();
  assert_eq!(out, get(inp, None));
}

#[test]
fn empty() {
  let inp = include_bytes!("test_inputs/empty/inp.txt");
  let out = inp.to_vec();
  assert_eq!(out, get(inp, None));
  for i in 1..=8 {
    assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
    assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
  }
}

#[test]
fn intervening_lines() {
  let inp = include_bytes!("test_inputs/intervening_lines/inp.txt");
  let out = include_bytes!("test_inputs/intervening_lines/out.txt").to_vec();
  assert_eq!(out, get(inp, None));
  for i in 1..=8 {
    assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
    assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
  }
}

#[test]
fn multi_lf_at_eof() {
  let inp = include_bytes!("test_inputs/multi_lf_at_eof/inp.txt");
  let out = include_bytes!("test_inputs/multi_lf_at_eof/out.txt").to_vec();
  assert_eq!(out, get(inp, None));
  for i in 1..=8 {
    assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
    assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
  }
}

#[test]
fn no_lf_at_eof() {
  let inp = include_bytes!("test_inputs/no_lf_at_eof/inp.txt");
  let out = include_bytes!("test_inputs/no_lf_at_eof/out.txt").to_vec();
  assert_eq!(out, get(inp, None));
  for i in 1..8 {
    assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
    assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
  }
}

#[test]
fn spaces_to_tabs() {
  let inp = include_bytes!("test_inputs/spaces_to_tabs/inp.txt");
  let out = include_bytes!("test_inputs/spaces_to_tabs/out.txt").to_vec();
  assert_eq!(out, get(inp, Some((Indent::Tabs, 2))));
}

#[test]
fn tabs_to_spaces() {
  let inp = include_bytes!("test_inputs/tabs_to_spaces/inp.txt");
  let out = include_bytes!("test_inputs/tabs_to_spaces/out.txt").to_vec();
  assert_eq!(out, get(inp, Some((Indent::Spaces, 2))));
}

#[test]
fn trailing() {
  let inp = include_bytes!("test_inputs/trailing/inp.txt");
  let out = include_bytes!("test_inputs/trailing/out.txt").to_vec();
  assert_eq!(out, get(inp, None));
  for i in 2..=8 {
    assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
    assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
  }
}
