#[derive(Debug)]
// TODO: implement proper error display
pub enum ScannerError {
	Unexpected {line: usize, message: String},
	NthCharNotFound,
}

pub type ScannerResult<T> = Result<T, ScannerError>;

impl ScannerError {
	pub fn report(&self) {
		println!("{:?}", self)
	}
}

