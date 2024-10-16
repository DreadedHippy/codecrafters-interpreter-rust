use error::{ScannerError, ScannerResult};
use token::{keywords, Literal, Token, TokenType};

use crate::{char_at, utils::{is_alpha, is_alphanumeric, substring}};

pub mod error;
pub mod token;

/// Lox Scanner
pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
  current: usize,
  line: usize,
	pub had_error: bool
}

impl Scanner {
	/// Create a new scanner
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
	
	/// Scan and return all file tokens
	pub fn scan_tokens(&mut self) -> ScannerResult<Vec<Token>> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token()?
		}

		self.tokens.push(Token::new(TokenType::EOF, "".to_string(), Literal::Null, self.line));

		Ok(self.tokens.clone())
	}

	pub fn error(&mut self, e: ScannerError) {
		self.had_error = true;
		e.report("");
	}

	/// Scan a file for a token
	fn scan_token(&mut self) -> ScannerResult<()> {
    let c = self.advance();
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
			// Double symbols
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
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				} else {
					self.add_token(TokenType::SLASH)
				}
			},
			// Whitespace
			' ' => {},
			'\r' => {},
			'\t' => {},
			'\n' => {self.line += 1},
			// String literals
			'"' => {
				self.string()
			},
			// Number literals
			'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
				self.number()
			}
			// Default
			c => {
				if is_alpha(c) {
					self.identifier()
				} else {

					self.error(
						ScannerError {
							line: self.line,
							message: format!("Unexpected character: {}", c)
						}
					)
				}
			}
    }

		return Ok(())
  }

	/// Check that the current char matches an expected char
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

	/// Tokenize a string
	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' { self.line += 1; }
			self.advance();
		}

		if self.is_at_end() {
			self.error(ScannerError { line: self.line, message: "Unterminated string.".to_string() });
			return;
		}

		self.advance();

		let value = substring(&self.source, self.start + 1, self.current - 1);
		self.add_token_to_list(TokenType::STRING, Literal::String(value.to_string()));
	}

	/// Tokenize a number
	fn number(&mut self) {
		while self.peek().is_digit(10) {
			self.advance();
		}

		if self.peek() == '.' && self.peek_next().is_digit(10) {
			self.advance();

			while self.peek().is_digit(10) { self.advance();}
		}

		self.add_token_to_list(TokenType::NUMBER, Literal::Float(substring(&self.source, self.start, self.current).parse::<f64>().unwrap()))

	}

	/// Tokenize an identifier
	fn identifier(&mut self) {
		while is_alphanumeric(self.peek()) { self.advance();};
		let text = substring(&self.source, self.start, self.current);

		let token_type = keywords().get(text).unwrap_or(&TokenType::IDENTIFIER).clone();

		self.add_token(token_type);
	}

	// Check the current character, without consuming
	fn peek(&self) -> char {
		if self.is_at_end() {
			return '\0';
		}
		
		return char_at(&self.source, self.current);
	}

	/// Check the next character, without consuming
	fn peek_next(&self) -> char {
		if self.current + 1 >= self.source.len() {
			return '\0'
		} 

		return char_at(&self.source, self.current + 1)
	}

	/// Consume and return the current character, move forward 1 step
	fn advance(&mut self) -> char {
		let c = char_at(&self.source, self.current);
		self.current += 1;

		return c;
	}

	/// Add a given token to the list with a literal of null
	fn add_token(&mut self, token_type: TokenType) {
		self.add_token_to_list(token_type, Literal::Null);
	}
	
	/// Add a given token to the list, with a given literal
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

	/// Check if is at end of source
	fn is_at_end(&self) -> bool {
		return self.current >= self.source.len()
	}
}



