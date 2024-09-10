use error::{ScannerError, ScannerResult};
use token::{Literal, Token, TokenType};

use crate::char_at;

pub mod error;
pub mod token;

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
  current: usize,
  line: usize,
	pub had_error: bool
}

impl Scanner {
	pub fn new(source: String) -> Self {
		Self {
			source,
			tokens: Vec::new(),
			start: 0,
			current: 0,
			line: 1,
			had_error: false
		}
	}
	
	pub fn scan_tokens(&mut self) -> ScannerResult<Vec<Token>> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token()?
		}

		self.tokens.push(Token::new(TokenType::EOF, "".to_string(), Literal::Null, self.line));

		Ok(self.tokens.clone())
	}

	fn error(&mut self, e: ScannerError) {
		self.had_error = true;
		e.report("");
	}

	fn scan_token(&mut self) -> ScannerResult<()> {
    let c = self.advance()?;
    match c {
      '(' => self.add_token(TokenType::LEFT_PAREN),
      ')' => self.add_token(TokenType::RIGHT_PAREN),
      '{' => self.add_token(TokenType::LEFT_BRACE),
      '}' => self.add_token(TokenType::RIGHT_BRACE),
      ',' => self.add_token(TokenType::COMMA),
      '.' => self.add_token(TokenType::DOT),
      '-' => self.add_token(TokenType::MINUS),
      '+' => self.add_token(TokenType::PLUS),
      ';' => self.add_token(TokenType::SEMICOLON),
      '*' => self.add_token(TokenType::STAR),
      '!' => {
				let c = if self.match_char('=') {TokenType::BANG_EQUAL} else {TokenType::BANG};
				self.add_token(c);
			},
			'=' => {
				let c = if self.match_char('=') {TokenType::EQUAL_EQUAL} else {TokenType::EQUAL};
				self.add_token(c);
			},
			'<' => {
				let c = if self.match_char('=') {TokenType::LESS_EQUAL} else {TokenType::LESS};
				self.add_token(c);
			},
			'>' => {
				let c = if self.match_char('=') {TokenType::GREATER_EQUAL} else {TokenType::GREATER};
				self.add_token(c);
			},
			'/' => {
				if self.match_char('/') {
					while self.peek() != Some('\n') && !self.is_at_end() {
						self.advance()?;
					}
				} else {
					self.add_token(TokenType::SLASH)
				}
			},
			' ' => {},
			'\r' => {},
			'\t' => {},
			'\n' => {self.line += 1},
			k => {
				self.error(
					ScannerError {
						line: self.line,
						message: format!("Unexpected character: {}", k)
					}
				)
			}
    }

		return Ok(())
  }

	fn match_char(&mut self, expected: char) -> bool{
		if self.is_at_end() {
			return false
		}

		let c = char_at(&self.source, self.current);

		if c != expected {
			return false
		}

		self.current += 1;
		return true

	}

	fn peek(&self) -> Option<char> {
		if self.is_at_end() {
			return Some('\0');
		}
		
		return Some(char_at(&self.source, self.current));
	}

	fn advance(&mut self) -> ScannerResult<char> {
		let c = char_at(&self.source, self.current);
		// .chars().nth(self.current).ok_or_else(|| ScannerError {line: self.line, message: "Nth char not found".to_string()});
		self.current += 1;

		return Ok(c);
	}

	fn add_token(&mut self, token_type: TokenType) {
		self.add_token_to_list(token_type, Literal::Null);
	}
	
	fn add_token_to_list(&mut self, token_type: TokenType, literal: Literal) {
		let text = &self.source[(self.start as usize)..(self.current as usize)];
		let token = Token {
			token_type,
			lexeme: text.to_string(),
			literal,
			line: self.line, 
		};

		self.tokens.push(token)
	}

	fn is_at_end(&self) -> bool {
		return self.current >= self.source.len()
	}
}



