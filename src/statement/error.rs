
use crate::{parser::error::ParserError, scanner::token::{Token, TokenType}};

pub struct StatementError {token: Token, message: String}

impl StatementError {
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

pub type StatementResult<T> = Result<T, StatementError>;

impl From<ParserError> for StatementError {
	fn from(value: ParserError) -> Self {
		Self{token: value.token, message: value.message}
	}
}