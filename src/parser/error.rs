use crate::scanner::token::{Token, TokenType};

/// A struct representing a Parser error
pub struct ParserError {
	pub token: Token,
	pub message: String
}

/// A wrapper result type for a Generic T and a ParserError
pub type ParserResult<T> = Result<T, ParserError>;

impl ParserError {
	/// Create a new parser error
	pub fn new(token: Token, message: String) -> Self {
		Self { token, message }
	}

	/// Construct an error report, and report it	
	pub fn error(&self) {
		if self.token.token_type == TokenType::EOF {
			self.report(" at end")
		} else {
			self.report(&format!(" at '{}'", self.token.lexeme))
		}
	}

	/// Report an error, given its location
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.token.line, where_, self.message);
	}
}