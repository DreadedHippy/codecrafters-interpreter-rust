/// Errors from the lox programming languagr
pub struct LoxError {
	pub line: usize,
	pub message: String
}

/// A wrapper type for a generic result and a Lox error.
pub type LoxResult<T> = Result<T, LoxError>;

impl LoxError {
	/// Create a new LoxError instance
	pub fn new(line: usize, message: String) -> Self {
		Self { line, message }
	}
	
	/// Display error to stderr
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.line, where_, self.message);
	}
}

