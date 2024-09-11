use std::str::from_utf8;

pub fn substring(string: &str, start: usize, end: usize) -> &str {
	from_utf8(&string.as_bytes()[start..end]).expect("Unable to convert u8 slice to valid utf8")
}

pub fn is_alpha(c: char) -> bool{
	(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
	c.is_digit(10) || is_alpha(c)
}