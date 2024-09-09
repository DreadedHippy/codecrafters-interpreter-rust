#[derive(Debug)]
// TODO: implement proper error display
pub struct ScannerError {
	pub line: usize,
	pub message: String
}

pub type ScannerResult<T> = Result<T, ScannerError>;

impl ScannerError {
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.line, where_, self.message);
	}
}

