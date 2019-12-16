//! The core logic.

pub fn get(bs: &[u8]) -> Vec<u8> {
	let mut ret = Vec::with_capacity(bs.len());
	let mut pending = Vec::new();
	for &b in bs {
		if b == b'\n' {
			pending.clear();
			ret.push(b);
			continue;
		}
		if b.is_ascii_whitespace() {
			pending.push(b);
		} else {
			ret.append(&mut pending);
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
