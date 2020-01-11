//! The core logic.

use crate::config::{Convert, Indent};

pub fn get(bs: &[u8], convert: Convert) -> Vec<u8> {
	let mut ret = Vec::with_capacity(bs.len());
	let mut cur_line_ws = Vec::new();
	let mut cur_line_just_ws = true;
	let mut new_lines = 0;
	for &b in bs {
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
		for _ in 0..new_lines {
			ret.push(b'\n');
		}
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

fn convert_to_spaces(bs: &[u8], n: usize) -> Vec<u8> {
	let mut ret = Vec::with_capacity(bs.len());
	for &b in bs {
		if b == b'\t' {
			for _ in 0..n {
				ret.push(b' ');
			}
			continue;
		}
		ret.push(b);
	}
	ret
}

fn convert_to_tabs(bs: &[u8], n: usize) -> Vec<u8> {
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
		for _ in 0..consec_spaces {
			ret.push(b' ');
		}
		consec_spaces = 0;
		ret.push(b);
	}
	for _ in 0..consec_spaces {
		ret.push(b' ');
	}
	ret.shrink_to_fit();
	ret
}

#[cfg(test)]
mod tests {
	use super::{get, Indent};

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
			println!("{}", i);
			assert_eq!(out, get(inp, Some((Indent::Spaces, i))));
			assert_eq!(out, get(inp, Some((Indent::Tabs, i))));
		}
	}
}
