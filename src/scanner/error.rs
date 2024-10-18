#[derive(Debug)]
// TODO: implement proper error display
/// Errors arising from Scanner operation
pub struct ScannerError {
	pub line: usize,
	pub message: String
}

/// Wrapper type for `Result<T, ScannerError>`
pub type ScannerResult<T> = Result<T, ScannerError>;

impl ScannerError {
	/// Print a scanner error to the stderr
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.line, where_, self.message);
	}
}

