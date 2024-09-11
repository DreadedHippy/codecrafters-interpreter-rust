use std::str::from_utf8;

pub fn substring(string: &str, start: usize, end: usize) -> &str {
	from_utf8(&string.as_bytes()[start..end]).expect("Unable to convert u8 slice to valid utf8")
}