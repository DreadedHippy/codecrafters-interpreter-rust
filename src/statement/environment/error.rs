
use crate::{parser::error::ParserError, scanner::token::{Token, TokenType}};

/// Errors resulting from operations with Environment
pub struct EnvironmentError {pub token: Token, pub message: String}

impl EnvironmentError {
	pub fn new(token: Token, message: &str) -> Self {
		Self {token, message: message.to_string()}
	}


	pub fn error(&self) {
		if self.token.token_type == TokenType::EOF {
			self.report(" at end")
		} else {
			self.report(&format!(" at '{}'", self.token.lexeme))
		}
	}

		
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.token.line, where_, self.message);
	}
}

/// Wrapper Type for `Result<T, EnvironmentError>`
pub type EnvironmentResult<T> = Result<T, EnvironmentError>;

impl From<ParserError> for EnvironmentError {
	fn from(value: ParserError) -> Self {
		Self{token: value.token, message: value.message}
	}
}