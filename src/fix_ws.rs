//! The core logic.

pub fn get(bs: &[u8]) -> Vec<u8> {
	let mut ret = Vec::with_capacity(bs.len());
	let mut cur_line = Vec::new();
	let mut new_lines = 0;
	for &b in bs {
		if b == b'\n' {
			cur_line.clear();
			new_lines += 1;
			continue;
		}
		if b.is_ascii_whitespace() {
			cur_line.push(b);
		} else {
			for _ in 0..new_lines {
				ret.push(b'\n');
			}
			new_lines = 0;
			ret.append(&mut cur_line);
			ret.push(b);
		}
	}
	if let Some(&b) = ret.last() {
		if b != b'\n' {
			ret.push(b'\n');
		}
	}
	ret
}

#[cfg(test)]
mod tests {
	use super::get;

	#[test]
	fn empty() {
		let inp = include_bytes!("test-inputs/empty/inp.txt");
		let out = inp.to_vec();
		assert_eq!(out, get(inp));
	}

	#[test]
	fn multi_nl_at_eof() {
		let inp = include_bytes!("test-inputs/multi-nl-at-eof/inp.txt");
		let out = include_bytes!("test-inputs/multi-nl-at-eof/out.txt").to_vec();
		assert_eq!(out, get(inp));
	}

	#[test]
	fn no_change() {
		let inp = include_bytes!("test-inputs/no-change/inp.txt");
		let out = inp.to_vec();
		assert_eq!(out, get(inp));
	}

	#[test]
	fn no_nl_at_eof() {
		let inp = include_bytes!("test-inputs/no-nl-at-eof/inp.txt");
		let out = include_bytes!("test-inputs/no-nl-at-eof/out.txt").to_vec();
		assert_eq!(out, get(inp));
	}

	#[test]
	fn trailing() {
		let inp = include_bytes!("test-inputs/trailing/inp.txt");
		let out = include_bytes!("test-inputs/trailing/out.txt").to_vec();
		assert_eq!(out, get(inp));
	}
}
