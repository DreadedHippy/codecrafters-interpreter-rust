#[derive(Clone, Debug)]
pub struct Token {
	pub token_type: TokenType,
	pub lexeme: String,
	pub literal: Literal,
	pub line: usize
}

impl Token {
	pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
		Self { token_type, lexeme, literal, line }
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
	}
}

#[derive(Debug, Clone)]
#[allow(unused, non_camel_case_types)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

// impl ToString for Token {
// 	fn to_string(&self) -> String {
// 		return format!("{} {} {}", self.token_type, self.lexeme, self.literal);
// 	}
// }

#[derive(Clone, Debug)]
pub enum Literal {
	Null,
	Integer(i64),
	String(String),
	Float(f64),
	Boolean(bool)
}

impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let as_str = match self {
			Self::Null => "null".to_string(),
			Self::Integer(x) => x.to_string(),
			Self::String(s) => s.to_string(),
			Self::Float(f) => f.to_string(),
			Self::Boolean(b) => b.to_string()
		};

		write!(f, "{}", as_str)
	}
}