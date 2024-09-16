use crate::scanner::token::{Token, TokenType};

pub struct ParserError {
	pub token: Token,
	pub message: String
}

pub type ParserResult<T> = Result<T, ParserError>;

impl ParserError {
	pub fn new(token: Token, message: String) -> Self {
		Self { token, message }
	}
	
	pub fn error(&mut self) {
		if self.token.token_type == TokenType::EOF {
			self.report("at end")
		} else {
			self.report(&format!("at '{}'", self.token.lexeme))
		}
	}

		
	pub fn report(&self, where_: &str) {
		eprintln!("[line {}] Error{}: {}", self.token.line, where_, self.message);
	}
}