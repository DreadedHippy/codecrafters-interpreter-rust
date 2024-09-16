use expr::{Expr, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary};
use error::{ParserError, ParserResult};

use crate::scanner::token::{Literal, Token, TokenType};

pub mod expr;
pub mod error;

pub struct Parser {
	pub tokens: Vec<Token>,
	current: usize,
	had_error: bool
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser {tokens, current: 0, had_error: false}
	}
}

impl Parser {
	pub fn parse(&mut self) -> Option<Expr> {
		self.expression().ok()
	}

	fn expression(&mut self) -> ParserResult<Expr> {
		return self.equality()
	}

	fn equality(&mut self) -> ParserResult<Expr> {
		let mut expr = self.comparison()?;

		while self.match_next(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
			let operator = self.previous();
			let right = self.comparison()?;

			expr = Expr::Binary(ExprBinary::new(expr, operator, right));
		}

		return Ok(expr);
	}

	fn match_next(&mut self, token_types: Vec<TokenType>) -> bool {
		for token_type in token_types {
			if self.check(token_type) {
				self.advance();
				return true
			}
		}

		return false
	}

	fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		return self.previous();
	}


	fn check(&self, token_type: TokenType) -> bool {
		if self.is_at_end() {return false}
		return self.peek().token_type == token_type
	}

	fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF
	}

	fn peek(&self) -> Token {
		return self.tokens.get(self.current).unwrap().clone()
	}

	fn previous(&self) -> Token {
		return self.tokens.get(self.current - 1).unwrap().clone()
	}


	// Comparison
	fn comparison(&mut self) -> ParserResult<Expr> {
		let mut expr = self.term()?;

		while self.match_next(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
			let operator = self.previous();
			let right = self.term()?;
			expr = Expr::Binary(ExprBinary::new(expr, operator, right))
		}

		Ok(expr)
	}

	fn term(&mut self) -> ParserResult<Expr> {
		let mut expr = self.factor()?;

		while self.match_next(vec![TokenType::MINUS, TokenType::PLUS]) {
			let operator = self.previous();
			let right = self.factor()?;

			match right {
				Expr::Literal(ExprLiteral::Null) => {return Err(self.error(self.peek(), format!("Invalid binary expression")))},
				_ => {}
			}
			expr = Expr::Binary(ExprBinary::new(expr, operator, right))
		}

		Ok(expr)
	}

	fn factor(&mut self) -> ParserResult<Expr> {
		let mut expr = self.unary()?;

		while self.match_next(vec![TokenType::SLASH, TokenType::STAR]) {
			let operator = self.previous();
			let right = self.unary()?;
			expr = Expr::Binary(ExprBinary::new(expr, operator, right))
		}

		Ok(expr)
	}

	fn unary(&mut self) -> ParserResult<Expr> {
		if self.match_next(vec![TokenType::BANG, TokenType::MINUS]) {
			let operator = self.previous();
			let right = self.unary()?;
			return Ok(Expr::Unary(ExprUnary {operator, right: Box::new(right)}))
		}


		return self.primary()
	}

	fn primary(&mut self) -> ParserResult<Expr> {
		if self.match_next(vec![TokenType::FALSE]) {return Ok(Expr::Literal(ExprLiteral::False))}
		if self.match_next(vec![TokenType::TRUE]) {return Ok(Expr::Literal(ExprLiteral::True))}
		if self.match_next(vec![TokenType::NIL]) {return Ok(Expr::Literal(ExprLiteral::Null))}

		if self.match_next(vec![TokenType::NUMBER]) {
			let v = match self.previous().literal {
				Literal::Float(x) => x,
				_ => 0.0
			};
			return Ok(Expr::Literal(ExprLiteral::NUMBER(v)))
		}

		if self.match_next(vec![TokenType::STRING]) {
			let v = match self.previous().literal {
				Literal::String(x) => x,
				_ => "".to_string()
			};
			return Ok(Expr::Literal(ExprLiteral::STRING(v)))
		}

		if self.match_next(vec![TokenType::LEFT_PAREN]) {
			let expr = self.expression()?;
			self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression".to_string())?;
			return Ok(Expr::Grouping(ExprGrouping(Box::new(expr))))
		}

		return Ok(Expr::Literal(ExprLiteral::Null))

	}

	fn consume(&mut self, token_type: TokenType, message: String) -> ParserResult<Token> {
		if self.check(token_type) {
			return Ok(self.advance())
		}

		return Err(self.error(self.peek(), message))
	}

	fn error(&mut self, token: Token, message: String) -> ParserError {
		self.had_error = true;
		let error = ParserError::new(token, message);
		error.error();
		error
	}









}
