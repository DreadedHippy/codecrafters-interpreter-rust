
pub struct LoxError {
	pub line: usize,
	pub message: String
}

pub type LoxResult<T> = Result<T, LoxError>;

impl LoxError {
	pub fn new(line: usize, message: String) -> Self {
		Self { line, message }
	}
	
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.line, where_, self.message);
	}
}

