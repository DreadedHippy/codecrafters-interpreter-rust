use error::{ScannerError, ScannerResult};
use token::{Literal, Token, TokenType};

pub mod error;
pub mod token;

pub fn tokenize(source: String) -> ScannerResult<Vec<Token>> {
	let mut scanner = Scanner::new(source);
	let tokens = scanner.scan_tokens()?;
	Ok(tokens)
}

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
	pub fn new(source: String) -> Self {
		Self {
			source,
			tokens: Vec::new(),
			start: 0,
			current: 0,
			line: 1
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
			_ => { return Err(ScannerError::Unexpected { line: self.line, message: "Unexpected character".to_string() })}
    }

		return Ok(())
  }

	fn advance(&mut self) -> ScannerResult<char> {
		let c = self.source.chars().nth(self.current).ok_or_else(|| ScannerError::NthCharNotFound);
		self.current += 1;

		return c;
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



