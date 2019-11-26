//! The core logic.

pub fn get(bs: Vec<u8>) -> Vec<u8> {
	let mut ret = Vec::with_capacity(bs.len());
	let mut pending = Vec::new();
	for b in bs {
		if b.is_ascii_whitespace() {
			pending.push(b);
		} else {
			ret.append(&mut pending);
			ret.push(b);
		}
	}
	ret
}
